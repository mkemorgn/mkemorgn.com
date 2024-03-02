document.addEventListener('DOMContentLoaded', function() {
  let navToggle = document.getElementById('navToggle');
  let navLinks = document.querySelector('.navbar-menu');

  navToggle.addEventListener('click', function() {
    navLinks.classList.toggle('is-active');
  });
});
