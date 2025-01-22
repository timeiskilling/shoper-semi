let cart = JSON.parse(localStorage.getItem('cart')) || [];

        function addToCart(name, price) {
            const item = { name, price: parseFloat(price) };
            cart.push(item);
            updateCart();
            saveCart();
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
                cartItem.innerHTML = `<p>${item.name}</p><p>${item.price} грн</p><button onclick="removeFromCart(${index})">Видалити</button>`;
                cartItems.appendChild(cartItem);
            });
            cartTotal.innerHTML = `Всього: ${total} грн`;
            cartCount.innerHTML = cart.length;
        }

        function saveCart() {
            localStorage.setItem('cart', JSON.stringify(cart));
        }

        function toggleCartModal() {
            const cartModal = document.getElementById('cartModal');
            cartModal.style.display = (cartModal.style.display === 'flex') ? 'none' : 'flex';
        }

        updateCart();