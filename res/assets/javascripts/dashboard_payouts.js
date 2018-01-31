var PayoutsManager = (function() {
  return {
    _last_page : 1,
    _is_loading : false,

    load_more : function() {
      if (PayoutsManager._is_loading === false) {
        PayoutsManager._is_loading = true;

        FormManager.submit();

        PayoutsManager._last_page++;

        PartialManager.load(
          ("/dashboard/payouts/partial/payouts/" +
            PayoutsManager._last_page + "/"),

          PayoutsManager.__handle_load_more_success,
          PayoutsManager.__handle_load_more_error
        );
      }
    },

    __handle_load_more_success : function(request) {
      var list_container = document.querySelector(".section-list");

      var result_list_items = (
        request.response.body.querySelectorAll(".section-list li")
      );
      var result_load_more = (
        request.response.body.querySelector(".section-box-more-wrap")
      );

      // Append results?
      if (result_list_items.length > 0) {
        for (var i = (result_list_items.length - 1); i >= 0; i--) {
          list_container.appendChild(result_list_items[i]);
        }
      }

      // Nuke load more?
      if (!result_load_more) {
        var load_more_container = (
          document.querySelector(".section-box-more-wrap")
        );

        if (load_more_container) {
          load_more_container.parentNode.removeChild(load_more_container);
        }
      }

      FormManager.unsubmit();

      // Not loading anymore
      PayoutsManager._is_loading = false;
    },

    __handle_load_more_error : function() {
      ToastManager.error(
        "Error loading more payouts.",
        "Older payouts could not be loaded. Try again."
      );

      FormManager.unsubmit();

      // Not loading anymore
      PayoutsManager._is_loading = false;
    }
  };
})();
