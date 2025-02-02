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

    const dropdownMenu = document.querySelector('.dropdown-menu');
    if (dropdownMenu.style.display === 'block') {
        dropdownMenu.style.display = 'none';
    }

    setTimeout(() => {
        window.location.href = url;
    }, 500);
}

document.addEventListener('DOMContentLoaded', () => {
    const content = document.getElementById('content');
    applySorting();
});

document.getElementById('authify-link').addEventListener('click', function(event) {
    if (window.location.pathname === '/') {
        event.preventDefault();
    }
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
        button.style.alignSelf = 'center';

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

// Save cart to localStorage
function saveCart() {
    localStorage.setItem('cart', JSON.stringify(cart));
}

// Toggle cart modal display
function toggleCartModal() {
    const cartModal = document.getElementById('cartModal');
    cartModal.style.display = (cartModal.style.display === 'flex') ? 'none' : 'flex';
}

// Validate search input
function validateSearch() {
    const searchInput = document.getElementById('searchInput').value.trim();
    return searchInput !== '';
}

// Dropdown menu functionality
document.addEventListener('DOMContentLoaded', () => {
    const dropdownButton = document.querySelector('.dropdown-button');
    const dropdownMenu = document.querySelector('.dropdown-menu');

    dropdownButton.addEventListener('click', () => {
        const isVisible = dropdownMenu.style.display === 'block';
        dropdownMenu.style.display = isVisible ? 'none' : 'block';
    });

    document.addEventListener('click', (event) => {
        if (!dropdownButton.contains(event.target) && !dropdownMenu.contains(event.target)) {
            dropdownMenu.style.display = 'none';
        }
    });
});

document.addEventListener('DOMContentLoaded', function() {
    fetch('/api/categories')
        .then(response => response.json())
        .then(categories => {
            const dropdownMenu = document.querySelector('.dropdown-menu');
            dropdownMenu.innerHTML = '';

            function createCategoryList(items) {
                const ul = document.createElement('ul');
                ul.className = 'subcategory-menu';

                items.forEach(item => {
                    const li = document.createElement('li');
                    li.className = 'dropdown-item';

                    const link = document.createElement('a');
                    link.href = `/category/${item.id}`;
                    link.textContent = item.name;
                    li.appendChild(link);

                    if (item.subcategories && item.subcategories.length > 0) {
                        li.appendChild(createCategoryList(item.subcategories));
                    }

                    ul.appendChild(li);
                });

                return ul;
            }

            function nestCategories(categories) {
                const map = {};
                const roots = [];

                categories.forEach(category => {
                    map[category.id] = { ...category, subcategories: [] };
                });

                categories.forEach(category => {
                    if (category.parent_id) {
                        map[category.parent_id].subcategories.push(map[category.id]);
                    } else {
                        roots.push(map[category.id]);
                    }
                });

                return roots;
            }

            const nestedCategories = nestCategories(categories);
            const categoryList = createCategoryList(nestedCategories);
            dropdownMenu.appendChild(categoryList);
        })
        .catch(error => {
            console.error('Error loading categories:', error);
        });
});


const openModalButton = document.getElementById('openModalButton');
const closeModalButton = document.getElementById('closeModalButton');
const modal = document.getElementById('addProductModal');





window.addEventListener('click', (event) => {
    if (event.target == modal) {
        modal.style.display = 'none';
        clearForm();
    }
});


// document.querySelectorAll('.custom-file-input input[type="file"]').forEach(input => {
//     const fileNameField = input.parentElement.querySelector('.file-name');

//     input.addEventListener('change', () => {
//         if (input.files.length > 1) {
//             fileNameField.textContent = `${input.files.length} files selected`;
//         } else if (input.files.length === 1) {
//             fileNameField.textContent = input.files[0].name;
//         } else {
//             fileNameField.textContent = 'No file selected';
//         }
//     });
// });


let selectedMainImage = null;
let selectedAdditionalImages = [];

function previewImages(input, previewContainerId, isMainImage = false) {
    const previewContainer = document.getElementById(previewContainerId);
    previewContainer.innerHTML = '';

    if (input.files && input.files.length > 0) {
        Array.from(input.files).forEach((file, index) => {
            const reader = new FileReader();
            reader.onload = e => {
                const imgContainer = document.createElement('div');
                imgContainer.classList.add('image-container');

                const img = document.createElement('img');
                img.src = e.target.result;
                img.alt = file.name;

                const removeButton = document.createElement('span');
                removeButton.textContent = '×';
                removeButton.classList.add('remove-image');
                removeButton.onclick = () => {
                    if (isMainImage) {
                        selectedMainImage = null;
                        document.getElementById('main_image').value = '';
                        previewImages(input, previewContainerId, true);
                    } else {
                        selectedAdditionalImages.splice(index, 1);
                        updateAdditionalImagesInput();
                        previewImages(input, previewContainerId);
                    }
                };

                imgContainer.appendChild(img);
                imgContainer.appendChild(removeButton);
                previewContainer.appendChild(imgContainer);
            };
            reader.readAsDataURL(file);
        });
    }
}

function updateAdditionalImagesInput() {
    const dataTransfer = new DataTransfer();
    selectedAdditionalImages.forEach(file => {
        dataTransfer.items.add(file);
    });
    document.getElementById('images').files = dataTransfer.files;

 
    const fileNameField = document.querySelector('#images').parentElement.querySelector('.file-name');
    if (selectedAdditionalImages.length > 1) {
        fileNameField.textContent = `${selectedAdditionalImages.length} files selected`;
    } else if (selectedAdditionalImages.length === 1) {
        fileNameField.textContent = selectedAdditionalImages[0].name;
    } else {
        fileNameField.textContent = 'No file selected';
    }
}


function clearForm() {
    const form = document.querySelector('#addProductModal form');
    form.reset();


    document.getElementById('mainImagePreview').innerHTML = '';
    document.getElementById('additionalImagesPreview').innerHTML = '';

 
    document.querySelectorAll('.file-name').forEach(fileNameField => {
        fileNameField.textContent = 'No file selected';
    });

    selectedMainImage = null;
    selectedAdditionalImages = [];
}

function applySorting() {
    const sortOption = document.getElementById('sort').value;
    const pageType = document.body.dataset.pageType;
    const categoryId = document.body.dataset.categoryId;

 

    let url = '';

    if (pageType === 'category' && categoryId) {
        url = `/category/${categoryId}/sort?how_sort=${sortOption}`;
    } else {
        url = `/sort?how_sort=${sortOption}`;
    }

    fetch(url)
        .then(response => response.json())
        .then(products => {
            displayProducts(products);
        })
        .catch(error => {
            console.error('Помилка при отриманні відсортованих товарів:', error);
        });
}
  

  function displayProducts(products) {
    const productList = document.querySelector('.product-list');
    productList.innerHTML = '';
  
    products.forEach(product => {
      const productItem = document.createElement('li');
      productItem.classList.add('product-item');
  
      productItem.innerHTML = `
        <a href="/product/${product.id}" class="product-link">
          <img src="/picture/${product.file_path}" alt="${product.name}" class="product-image">
        </a>
        <div class="product-details">
          <a href="/product/${product.id}" class="product-link">
            <strong class="product-name">${product.name}</strong>
          </a>
          <span class="product-price">${product.price} грн</span>
          <p class="product-description">${product.description}</p>
        </div>
      `;
  
      productList.appendChild(productItem);
    });
  }