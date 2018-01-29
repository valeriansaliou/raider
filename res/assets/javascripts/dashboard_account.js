var PasswordAccountManager = (function() {
  return {
    change : function() {
      var field_selector = document.getElementById("account-password-field"),
          link_selector = document.getElementById("account-password-link");

      link_selector.style.display = "none";

      field_selector.style.display = "block";
      field_selector.focus();
    }
  };
})();
