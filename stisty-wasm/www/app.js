import init, { analyze_genome, lookup_snp, chromosome_stats } from './stisty_wasm.js';

// State
let genomeData = null;
let summaryData = null;

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

        // Read file content
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
                        <value>${result.rsid}</value>
                    </div>
                    <div class="snp-field">
                        <label>Chromosome</label>
                        <value>${result.chromosome}</value>
                    </div>
                    <div class="snp-field">
                        <label>Position</label>
                        <value>${result.position.toLocaleString()}</value>
                    </div>
                    <div class="snp-field">
                        <label>Genotype</label>
                        <value>${result.genotype}</value>
                    </div>
                    <div class="snp-field">
                        <label>Type</label>
                        <value>${result.is_heterozygous ? 'Heterozygous' : 'Homozygous'}</value>
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
                    <value>${result.chromosome}</value>
                </div>
                <div class="snp-field">
                    <label>Total SNPs</label>
                    <value>${result.total_snps.toLocaleString()}</value>
                </div>
                <div class="snp-field">
                    <label>Heterozygous SNPs</label>
                    <value>${result.heterozygous_count.toLocaleString()}</value>
                </div>
                <div class="snp-field">
                    <label>Heterozygosity Rate</label>
                    <value>${hetRate}%</value>
                </div>
            </div>
        `;

        chrResults.classList.remove('hidden');
    } catch (error) {
        console.error('Chromosome stats error:', error);
        alert(`Failed to get chromosome statistics: ${error.message}`);
    }
});

// Initialize
initWasm();