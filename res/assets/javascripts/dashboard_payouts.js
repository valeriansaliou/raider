var PayoutsManager = (function() {
  return {
    _last_page : 1,
    _is_loading : false,

    load_more : function() {
      if (PayoutsManager._is_loading === false) {
        PayoutsManager._is_loading = true;

        PayoutsManager._last_page++;

        // TODO: set loading state on load button
        // TODO: load next page
        // TODO: unlock once loaded

        // TODO
        console.error("load page", PayoutsManager._last_page);
      }
    }
  };
})();
