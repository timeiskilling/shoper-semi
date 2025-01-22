function toggleMenu() {
    const navButtons = document.querySelector('.nav-buttons');
    if (navButtons.style.display === 'flex') {
        navButtons.style.display = 'none';
    } else {
        navButtons.style.display = 'flex';
    }
}