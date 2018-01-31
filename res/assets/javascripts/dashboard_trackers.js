var FormTrackersManager = (function() {
  return {
    _count_selected : 0,

    remove_trackers_confirm : function() {
      IntentManager.show("modal", "remove");
    },

    remove_trackers_submit : function() {
      IntentManager.hide("modal", "remove");

      FormManager.submit();

      document.getElementById("trackers-remove-form").submit();
    },

    create_tracker : function() {
      var modal_selector = IntentManager.show("modal", "create");

      if (modal_selector) {
        modal_selector.querySelector("input").focus();
      }
    },

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

      document.getElementById("trackers-remove-button").setAttribute(
        "data-locked",
        ((FormTrackersManager._count_selected > 0) ? "false" : "true")
      );
    },

    copy_tracker_link : function(button) {
      var link_selector = button.parentElement.parentElement.querySelector(
        "input.item-identity-sub"
      );

      if (link_selector) {
        link_selector.select();

        document.execCommand("copy");

        ToastManager.info(
          "Tracker link copied.",
          "The tracker link has been copied to your clipboard."
        );
      }
    }
  };
})();


var FormBannersManager = (function() {
  return {
    _last_tracker_link : "",
    _attribute_escape_regex : /"/g,

    open : function(button) {
      var link_selector = button.parentElement.parentElement.querySelector(
        "input.item-identity-sub"
      );

      if (link_selector) {
        FormBannersManager._last_tracker_link = link_selector.value;

        var popup_selector = document.querySelector(".popup-lock");

        if (popup_selector) {
          var selected_picker_selector = popup_selector.querySelector(
            ".popup-banner-picker li[data-selected=\"true\"] a"
          );

          if (selected_picker_selector) {
            // Re-generate banner HTML code
            FormBannersManager.__generate_html(selected_picker_selector);
          }

          IntentManager.show("popup", "banner");
        }
      }
    },

    pick_banner : function(banner) {
      var popup_selector = document.querySelector(".popup-lock");

      if (popup_selector) {
        // Select target elements
        var pickers_selector = (
          popup_selector.querySelectorAll(".popup-banner-picker li")
        );
        var next_selector = (
          popup_selector.querySelector(".popup-actions .button-next")
        );

        // Select picked banner
        for (var i = 0; i < pickers_selector.length; i++) {
          if (pickers_selector[i].getAttribute("data-selected") !== "false") {
            pickers_selector[i].setAttribute("data-selected", "false");
          }
        }

        banner.parentElement.setAttribute("data-selected", "true");

        // Generate banner HTML code
        FormBannersManager.__generate_html(banner);

        // Activate copy button
        next_selector.removeAttribute("data-locked");
      }
    },

    copy_selected_banner : function() {
      document.getElementById("trackers-banner-code").select();
      document.execCommand("copy");

      IntentManager.hide("popup", "banner");

      ToastManager.info(
        "Banner HTML code copied.",
        "The banner HTML code has been copied to your clipboard for tracker."
      );
    },

    __generate_html : function(banner) {
      // Generate banner HTML code
      var image_selector = banner.querySelector("img");
      var code_selector = document.getElementById("trackers-banner-code");

      var html_data = {
        href   : FormBannersManager.__attribute(
          FormBannersManager._last_tracker_link
        ),
        src    : FormBannersManager.__attribute(
          image_selector.getAttribute("src")
        ),
        width  : FormBannersManager.__attribute(
          image_selector.getAttribute("data-width")
        ),
        height : FormBannersManager.__attribute(
          image_selector.getAttribute("data-height")
        )
      };

      code_selector.value = (
        "<a href=\"" + html_data.href + "\" target=\"_blank\">" +
          "<img src=\"" + html_data.src + "\""     +
            " width=\"" + html_data.width + "\""   +
            " height=\"" + html_data.height + "\"" +
            " alt=\"\" />" +
        "</a>"
      );
    },

    __attribute : function(value) {
      return (value || "").replace(
        FormBannersManager._attribute_escape_regex, "&quot;"
      );
    }
  };
})();
