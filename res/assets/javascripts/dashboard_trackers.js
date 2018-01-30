var FormTrackersManager = (function() {
  return {
    checkbox_change : function(checkbox) {
      checkbox.parentElement.parentElement.setAttribute(
        "data-selected", ((checkbox.checked === true) ? "true" : "false")
      );
    }
  };
})();
