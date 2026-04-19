document.addEventListener('DOMContentLoaded', () => {
    // Simulated live ledger updates
    const ledgerContent = document.getElementById('ledger-content');
    const statuses = ['PENDING', 'PREPARING', 'SHIPPING', 'DELIVERED'];
    const users = ['Farmer Ali', 'Basmaty Mfg', 'Wza3ly Log', 'Carefour Ret', 'Mera Consumer'];

    const addRandomTx = () => {
        const from = users[Math.floor(Math.random() * (users.length - 1))];
        const to = users[Math.floor(Math.random() * (users.length - 1)) + 1];
        const status = statuses[Math.floor(Math.random() * statuses.length)];
        const hash = Math.random().toString(16).substring(2, 10) + '...';
        const time = Date.now().toString().substring(5);

        const row = document.createElement('div');
        row.className = 'tx-row';
        row.style.opacity = '0';
        row.style.transform = 'translateY(-10px)';
        row.innerHTML = `
            <span class="hash-text">${hash}</span>
            <span>${from} → ${to}</span>
            <span class="status-badge status-${status.toLowerCase()}">${status}</span>
            <span>${time}</span>
        `;
        
        ledgerContent.prepend(row);
        
        // Final styling
        setTimeout(() => {
            row.style.transition = 'all 0.5s ease';
            row.style.opacity = '1';
            row.style.transform = 'translateY(0)';
        }, 100);

        // Keep list clean
        if (ledgerContent.children.length > 8) {
            ledgerContent.lastElementChild.remove();
        }
    };

    // Auto-update every few seconds
    setInterval(addRandomTx, 4000);

    // Initial load effects for stats
    const stats = document.querySelectorAll('.value');
    stats.forEach(stat => {
        const target = parseInt(stat.textContent);
        if (isNaN(target)) return;
        let count = 0;
        const speed = 2000 / target;
        
        const updateCount = () => {
            if (count < target) {
                count++;
                stat.textContent = count;
                setTimeout(updateCount, speed);
            }
        };
        updateCount();
    });
});
