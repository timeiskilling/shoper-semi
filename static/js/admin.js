document.addEventListener('DOMContentLoaded', function () {
  const addProductModal = document.getElementById('addProductModal');
  const openModalButton = document.getElementById('openModalButton');
  const closeModalButton = document.getElementById('closeModalButton');
  
  if (openModalButton && addProductModal) {
      openModalButton.addEventListener('click', function () {
          addProductModal.style.display = 'block';
      });
  }
  
  if (closeModalButton && addProductModal) {
      closeModalButton.addEventListener('click', function () {
          addProductModal.style.display = 'none';
          clearForm();
      });
  }
  
  window.addEventListener('click', function (event) {
      if (event.target === addProductModal) {
          addProductModal.style.display = 'none';
          clearForm();
      }
  });

  function clearForm() {
      const form = document.querySelector('#addProductModal form');
      if (form) {
          form.reset();
      }

      document.getElementById('mainImagePreview').innerHTML = '';
      document.getElementById('additionalImagesPreview').innerHTML = '';
  }

  document.getElementById('main_image').addEventListener('change', function () {
      previewImages(this, 'mainImagePreview');
  });

  document.getElementById('images').addEventListener('change', function () {
      previewImages(this, 'additionalImagesPreview');
  });

  function previewImages(input, previewContainerId) {
      const previewContainer = document.getElementById(previewContainerId);
      previewContainer.innerHTML = '';

      if (input.files && input.files.length > 0) {
          Array.from(input.files).forEach(file => {
              const reader = new FileReader();
              reader.onload = e => {
                  const img = document.createElement('img');
                  img.src = e.target.result;
                  img.alt = file.name;
                  previewContainer.appendChild(img);
              };
              reader.readAsDataURL(file);
          });
      }
  }
});
