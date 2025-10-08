import init, { analyze_genome, lookup_snp, chromosome_stats, generate_vcf } from './stisty_wasm.js';

// ============================================================================
// PRIVACY NOTICE
// ============================================================================
// This application processes ALL data CLIENT-SIDE in your browser.
//
// IMPORTANT PRIVACY GUARANTEES:
// - NO data is ever sent to any server
// - NO network requests are made with your genetic data
// - NO analytics or tracking of genetic information
// - All processing happens locally in WebAssembly
// - File reading uses browser FileReader API (local only)
// - Downloads use Blob URLs (local browser memory only)
//
// Your genetic data never leaves your device. Period.
// ============================================================================

// State - All stored in browser memory only, never transmitted
let genomeData = null;
let summaryData = null;
let vcfData = null;

// Initialize WASM module
async function initWasm() {
    try {
        await init();
        console.log('✅ WASM module initialized');
    } catch (error) {
        console.error('❌ Failed to initialize WASM:', error);
        alert('Failed to initialize the application. Please refresh the page.');
    }
}

// File handling
const fileInput = document.getElementById('fileInput');
const fileName = document.getElementById('fileName');
const loadingIndicator = document.getElementById('loadingIndicator');
const resultsSection = document.getElementById('resultsSection');

fileInput.addEventListener('change', async (event) => {
    const file = event.target.files[0];
    if (!file) return;

    fileName.textContent = file.name;

    try {
        loadingIndicator.classList.remove('hidden');
        resultsSection.classList.add('hidden');

        // PRIVACY: Read file content locally using browser FileReader API
        // This file content NEVER leaves your browser
        const text = await file.text();
        genomeData = text;

        // Analyze genome
        const resultJson = analyze_genome(text);
        summaryData = JSON.parse(resultJson);

        // Display results
        displaySummary(summaryData);

        loadingIndicator.classList.add('hidden');
        resultsSection.classList.remove('hidden');
    } catch (error) {
        console.error('Analysis error:', error);
        alert(`Failed to analyze genome data: ${error.message}`);
        loadingIndicator.classList.add('hidden');
    }
});

// Display summary
function displaySummary(data) {
    // Update stats
    document.getElementById('totalSnps').textContent = data.total_snps.toLocaleString();

    const hetRate = (data.heterozygosity_rate * 100).toFixed(2);
    document.getElementById('hetRate').textContent = `${hetRate}%`;

    document.getElementById('tsTvRatio').textContent = data.ts_tv_ratio.toFixed(4);

    // Display allele frequencies
    displayAlleleChart(data.allele_frequencies);

    // Display chromosome counts
    displayChromosomeChart(data.chromosome_counts);
}

// Display allele frequency bar chart
function displayAlleleChart(frequencies) {
    const container = document.getElementById('alleleChart');
    container.innerHTML = '';

    const sortedFreqs = frequencies.sort((a, b) => b[1] - a[1]);
    const maxFreq = Math.max(...sortedFreqs.map(f => f[1]));

    const chart = document.createElement('div');
    chart.className = 'bar-chart';

    sortedFreqs.forEach(([allele, freq]) => {
        const percentage = (freq * 100).toFixed(2);
        const barWidth = (freq / maxFreq * 100).toFixed(1);

        const item = document.createElement('div');
        item.className = 'bar-item';
        item.innerHTML = `
            <div class="bar-label">${allele}</div>
            <div class="bar-container">
                <div class="bar-fill" style="width: ${barWidth}%">${percentage}%</div>
            </div>
        `;
        chart.appendChild(item);
    });

    container.appendChild(chart);
}

// Display chromosome counts bar chart
function displayChromosomeChart(counts) {
    const container = document.getElementById('chromosomeChart');
    container.innerHTML = '';

    // Sort chromosomes numerically
    const sortedCounts = counts.sort((a, b) => {
        const aNum = parseInt(a[0]);
        const bNum = parseInt(b[0]);
        if (!isNaN(aNum) && !isNaN(bNum)) {
            return aNum - bNum;
        }
        return a[0].localeCompare(b[0]);
    });

    const maxCount = Math.max(...sortedCounts.map(c => c[1]));

    const chart = document.createElement('div');
    chart.className = 'bar-chart';

    sortedCounts.forEach(([chr, count]) => {
        const barWidth = (count / maxCount * 100).toFixed(1);

        const item = document.createElement('div');
        item.className = 'bar-item';
        item.innerHTML = `
            <div class="bar-label">Chr ${chr}</div>
            <div class="bar-container">
                <div class="bar-fill" style="width: ${barWidth}%">${count.toLocaleString()}</div>
            </div>
        `;
        chart.appendChild(item);
    });

    container.appendChild(chart);
}

// Tab switching
const tabButtons = document.querySelectorAll('.tab-button');
const tabContents = document.querySelectorAll('.tab-content');

tabButtons.forEach(button => {
    button.addEventListener('click', () => {
        const tabName = button.dataset.tab;

        // Update active states
        tabButtons.forEach(btn => btn.classList.remove('active'));
        tabContents.forEach(content => content.classList.remove('active'));

        button.classList.add('active');
        document.getElementById(`${tabName}Tab`).classList.add('active');
    });
});

// SNP Lookup
const snpInput = document.getElementById('snpInput');
const lookupButton = document.getElementById('lookupButton');
const snpResults = document.getElementById('snpResults');

lookupButton.addEventListener('click', async () => {
    if (!genomeData) {
        alert('Please upload a genome file first');
        return;
    }

    const rsid = snpInput.value.trim();
    if (!rsid) {
        alert('Please enter an SNP rsid');
        return;
    }

    try {
        const resultJson = lookup_snp(genomeData, rsid);
        const result = JSON.parse(resultJson);

        if (result === null) {
            snpResults.innerHTML = '<p style="color: var(--warning-color);">SNP not found in your genome data.</p>';
        } else {
            snpResults.innerHTML = `
                <div class="snp-info">
                    <div class="snp-field">
                        <label>rsID</label>
                        <div class="value">${result.rsid}</div>
                    </div>
                    <div class="snp-field">
                        <label>Chromosome</label>
                        <div class="value">${result.chromosome}</div>
                    </div>
                    <div class="snp-field">
                        <label>Position</label>
                        <div class="value">${result.position.toLocaleString()}</div>
                    </div>
                    <div class="snp-field">
                        <label>Genotype</label>
                        <div class="value">${result.genotype}</div>
                    </div>
                    <div class="snp-field">
                        <label>Type</label>
                        <div class="value">${result.is_heterozygous ? 'Heterozygous' : 'Homozygous'}</div>
                    </div>
                </div>
            `;
        }

        snpResults.classList.remove('hidden');
    } catch (error) {
        console.error('Lookup error:', error);
        alert(`Failed to lookup SNP: ${error.message}`);
    }
});

// Enter key for SNP lookup
snpInput.addEventListener('keypress', (event) => {
    if (event.key === 'Enter') {
        lookupButton.click();
    }
});

// Chromosome stats
const chrSelect = document.getElementById('chrSelect');
const chrResults = document.getElementById('chrResults');

chrSelect.addEventListener('change', async () => {
    if (!genomeData) {
        alert('Please upload a genome file first');
        return;
    }

    const chr = chrSelect.value;
    if (!chr) {
        chrResults.classList.add('hidden');
        return;
    }

    try {
        const resultJson = chromosome_stats(genomeData, chr);
        const result = JSON.parse(resultJson);

        const hetRate = (result.heterozygosity_rate * 100).toFixed(2);

        chrResults.innerHTML = `
            <div class="snp-info">
                <div class="snp-field">
                    <label>Chromosome</label>
                    <div class="value">${result.chromosome}</div>
                </div>
                <div class="snp-field">
                    <label>Total SNPs</label>
                    <div class="value">${result.total_snps.toLocaleString()}</div>
                </div>
                <div class="snp-field">
                    <label>Heterozygous SNPs</label>
                    <div class="value">${result.heterozygous_count.toLocaleString()}</div>
                </div>
                <div class="snp-field">
                    <label>Heterozygosity Rate</label>
                    <div class="value">${hetRate}%</div>
                </div>
            </div>
        `;

        chrResults.classList.remove('hidden');
    } catch (error) {
        console.error('Chromosome stats error:', error);
        alert(`Failed to get chromosome statistics: ${error.message}`);
    }
});

// VCF Export
const generateVcfButton = document.getElementById('generateVcfButton');
const downloadVcfButton = document.getElementById('downloadVcfButton');
const showVcfButton = document.getElementById('showVcfButton');
const hideVcfButton = document.getElementById('hideVcfButton');
const vcfChrSelect = document.getElementById('vcfChrSelect');
const vcfLoading = document.getElementById('vcfLoading');
const vcfSuccess = document.getElementById('vcfSuccess');
const vcfDisplay = document.getElementById('vcfDisplay');
const vcfContent = document.getElementById('vcfContent');

generateVcfButton.addEventListener('click', async () => {
    if (!genomeData) {
        alert('Please upload a genome file first');
        return;
    }

    try {
        vcfLoading.classList.remove('hidden');
        vcfSuccess.classList.add('hidden');
        vcfDisplay.classList.add('hidden');

        const chromosome = vcfChrSelect.value; // Empty string for all chromosomes
        vcfData = generate_vcf(genomeData, chromosome);

        vcfLoading.classList.add('hidden');
        vcfSuccess.classList.remove('hidden');
    } catch (error) {
        console.error('VCF generation error:', error);
        const chrContext = vcfChrSelect.value ? ` for chromosome ${vcfChrSelect.value}` : ' for all chromosomes';
        alert(`Failed to generate VCF${chrContext}: ${error.message}`);
        vcfLoading.classList.add('hidden');
    }
});

showVcfButton.addEventListener('click', () => {
    if (!vcfData) {
        alert('Please generate VCF first');
        return;
    }

    // Display the VCF content
    vcfContent.textContent = vcfData;
    vcfDisplay.classList.remove('hidden');

    // Scroll to the display
    vcfDisplay.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
});

hideVcfButton.addEventListener('click', () => {
    vcfDisplay.classList.add('hidden');
});

downloadVcfButton.addEventListener('click', () => {
    if (!vcfData) {
        alert('Please generate VCF first');
        return;
    }

    // PRIVACY: Create a Blob URL from local browser memory only
    // No server upload - file is downloaded directly from your browser
    const blob = new Blob([vcfData], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);

    // Create a temporary link and trigger download
    const a = document.createElement('a');
    a.href = url;

    // Generate filename based on chromosome selection
    const chr = vcfChrSelect.value;
    const chrSuffix = chr ? `_chr${chr}` : '_all';
    a.download = `genome${chrSuffix}.vcf`;

    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);

    // Clean up the URL object
    URL.revokeObjectURL(url);
});

// Initialize
initWasm();