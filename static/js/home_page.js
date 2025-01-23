function toggleMenu() {
    const navButtons = document.querySelector('.nav-buttons');
    if (navButtons.style.display === 'flex') {
        navButtons.style.display = 'none';
    } else {
        navButtons.style.display = 'flex';
    }
}

function performSearch() {
    const query = document.getElementById('search').value;
    if (query.trim() === "") {
        return;
    }
    window.location.href = `/search?q=${query}`;
}

function handleKeyPress(event) {
    if (event.key === 'Enter') {
        performSearch();
    }
}

function navigate(url) {
    const content = document.getElementById('content');
    content.classList.add('fade-leave-active');
    setTimeout(() => {
        window.location.href = url;
    }, 500);
}

document.addEventListener('DOMContentLoaded', () => {
    const content = document.getElementById('content');
    content.classList.add('fade-enter-active');
});

document.getElementById('authify-link').addEventListener('click', function(event) {
    if (window.location.pathname === '/') {
        event.preventDefault();
    }
});

function navigate(url) {
    const content = document.getElementById('content');
    content.classList.add('fade-leave-active');
    setTimeout(() => {
        window.location.href = url;
    }, 500);
}

document.addEventListener('DOMContentLoaded', () => {
    const content = document.getElementById('content');
    content.classList.add('fade-enter-active');
});

let cart = JSON.parse(localStorage.getItem('cart')) || [];

function addToCart(name, price, filePath) {
    const item = { name, price: parseFloat(price), filePath };
    cart.push(item);
    updateCart();
    saveCart();
    toggleCartModal(); 
}

function removeFromCart(index) {
    cart.splice(index, 1);
    updateCart();
    saveCart();
}

function updateCart() {
    const cartItems = document.getElementById('cartItems');
    const cartTotal = document.getElementById('cartTotal');
    const cartCount = document.getElementById('cartCount');
    cartItems.innerHTML = '';
    let total = 0;
    cart.forEach((item, index) => {
        total += item.price;
        const cartItem = document.createElement('div');
        cartItem.classList.add('cart-item');
        cartItem.innerHTML = `
            <img src="/picture/${item.filePath}" alt="${item.name}" class="cart-item-image">
            <div class="cart-item-info">
                <p>${item.name}</p>
                <p class="cart-item-price">${item.price} грн</p>
                <button class="remove-button" onclick="removeFromCart(${index})">Видалити</button>
            </div>
        `;
        cartItems.appendChild(cartItem);
    });

    // Додавання стилів для кнопки "Видалити"
    const removeButtons = document.querySelectorAll('.remove-button');
    removeButtons.forEach(button => {
        button.style.backgroundColor = '#333';
        button.style.border = 'none';
        button.style.color = 'white';
        button.style.padding = '10px 20px';
        button.style.borderRadius = '5px';
        button.style.cursor = 'pointer';
        button.style.marginTop = '0';
        button.style.transition = 'background-color 0.3s';
        button.style.alignSelf = 'center'; // Вирівнювання по центру вертикально

        button.onmouseover = () => {
            button.style.backgroundColor = '#555';
        };

        button.onmouseout = () => {
            button.style.backgroundColor = '#333';
        };
    });

    cartTotal.innerHTML = `Всього: ${total} грн`;
    cartCount.innerHTML = cart.length;
}

updateCart();



function saveCart() {
    localStorage.setItem('cart', JSON.stringify(cart));
}

function toggleCartModal() {
    const cartModal = document.getElementById('cartModal');
    cartModal.style.display = (cartModal.style.display === 'flex') ? 'none' : 'flex';
}

