var FormTrackersManager = (function() {
  return {
    _count_selected : 0,

    checkbox_change : function(checkbox) {
      FormTrackersManager._count_selected = Math.max(
        0,

        (
          FormTrackersManager._count_selected +
            ((checkbox.checked === true) ? 1 : -1)
        )
      );

      checkbox.parentElement.parentElement.setAttribute(
        "data-selected", ((checkbox.checked === true) ? "true" : "false")
      );

      document.getElementById("trackers-remove").setAttribute(
        "data-locked",
        ((FormTrackersManager._count_selected > 0) ? "false" : "true")
      );
    }
  };
})();
