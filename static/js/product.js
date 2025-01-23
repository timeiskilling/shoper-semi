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


