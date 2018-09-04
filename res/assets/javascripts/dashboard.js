var ToastManager = (function() {
  return {
    _toast : {},
    _selectors : {},

    _TOAST_CLOSE_DELAY : 250,
    _TOAST_OPEN_DELAY : 350,

    _TOAST_AUTO_CLOSE_DELAY_DEFAULT : 8000,
    _TOAST_AUTO_CLOSE_DELAY_SHORT : 2000,

    _TOAST_MESSAGE_FORMAT_SPACE_REGEX : /[_-]+/g,

    _toast_close_timeout : null,
    _toast_open_timeout : null,
    _toast_auto_close_timeout : null,

    success : function(label, message) {
      ToastManager.__open("success", label, message);
    },

    info : function(label, message) {
      ToastManager.__open("info", label, message);
    },

    warning : function(label, message) {
      ToastManager.__open("warning", label, message);
    },

    error : function(label, message) {
      ToastManager.__open("error", label, message);
    },

    close : function() {
      if (ToastManager._toast.active === true) {
        // Cancel previous toast close?
        if (ToastManager._toast_close_timeout !== null) {
          clearTimeout(ToastManager._toast_close_timeout);

          ToastManager._toast_close_timeout = null;
        }

        // Mark toast as inactive
        ToastManager._toast.active = false;

        // Hide toast
        ToastManager.__select(".toast").setAttribute("data-active", "false");

        // Close toast
        ToastManager._toast_close_timeout = setTimeout(function() {
          // Unschedule close (eg. if still scheduled)
          ToastManager.__unschedule_close();
        }, ToastManager._TOAST_CLOSE_DELAY);
      }

      return false;
    },

    _mouse_over : function() {
      // Unschedules active closes
      ToastManager.__unschedule_close();
    },

    _mouse_leave : function() {
      // Re-schedule toast auto-close
      ToastManager.__schedule_close(true);
    },

    __open : function(level, label, message) {
      if (!label) {
        throw new Error("No label provided for toast");
      }
      if (!message) {
        throw new Error("No message provided for toast");
      }

      // Cancel previous toast open?
      if (ToastManager._toast_open_timeout !== null) {
        clearTimeout(ToastManager._toast_open_timeout);

        ToastManager._toast_open_timeout = null;
      }

      // Close previous toast?
      ToastManager.close();

      // Open toast
      ToastManager._toast_open_timeout = setTimeout(function() {
        ToastManager._toast_open_timeout = null;

        // Mark toast as active
        ToastManager._toast.active = true;

        // Assign toast state
        ToastManager.__select(".toast-view-wrapped-message-main").innerText = (
          label
        );
        ToastManager.__select(".toast-view-wrapped-message-sub").innerText = (
          message
        );

        // Show toast
        var toast_sel = ToastManager.__select(".toast");

        toast_sel.setAttribute("data-active", "true");
        toast_sel.setAttribute("data-level", level);

        // Schedule toast auto-close
        ToastManager.__schedule_close();
      }, ToastManager._TOAST_OPEN_DELAY);

      return false;
    },

    __schedule_close : function(is_short) {
      // Unschedule any previous close
      ToastManager.__unschedule_close();

      // Schedule auto close
      ToastManager._toast_auto_close_timeout = setTimeout(function() {
        ToastManager._toast_auto_close_timeout = null;

        ToastManager.close();
      }, (
        (is_short === true) ? ToastManager._TOAST_AUTO_CLOSE_DELAY_SHORT :
          ToastManager._TOAST_AUTO_CLOSE_DELAY_DEFAULT
      ));
    },

    __unschedule_close : function() {
      if (ToastManager._toast_auto_close_timeout !== null) {
        clearTimeout(ToastManager._toast_auto_close_timeout);

        ToastManager._toast_auto_close_timeout = null;
      }
    },

    __format_message : function(message) {
      // Convert all space-like chars to actual spaces
      message = (
        message.replace(
          ToastManager._TOAST_MESSAGE_FORMAT_SPACE_REGEX, " "
        ).trim()
      );

      // Capitalize first letter from message
      message = (message.charAt(0).toUpperCase() + message.slice(1));

      return message;
    },

    __select : function(target) {
      ToastManager._selectors[target] = (
        ToastManager._selectors[target] || document.querySelector(target)
      );

      return ToastManager._selectors[target];
    }
  };
})();


var IntentManager = (function() {
  return {
    show : function(type, name, return_selector) {
      var target_selector = this.__visibility(type, name, true);

      return (
        (return_selector === true) ? target_selector : null
      );
    },

    hide : function(type, name, return_selector) {
      var target_selector = this.__visibility(type, name, false);

      return (
        (return_selector === true) ? target_selector : null
      );
    },

    __visibility : function(type, name, is_visible) {
      var target_selector = (
        document.querySelector("." + type + "-lock[data-name=\"" + name + "\"]")
      );

      if (target_selector) {
        target_selector.setAttribute(
          "data-visible", ((is_visible === true) ? "true" : "false")
        );

        return target_selector;
      }

      return null;
    },
  };
})();


var FormManager = (function() {
  return {
    submit : function() {
      FormManager.__toggle(true);
    },

    unsubmit : function() {
      FormManager.__toggle(false);
    },

    __toggle : function(is_pending) {
      document.querySelector("main").setAttribute("data-pending", (
        (is_pending === true) ? "true" : "false"
      ));
    }
  };
})();


var PartialManager = (function() {
  return {
    load : function(path, fn_handle_done, fn_handle_error) {
      var request = new XMLHttpRequest();

      request.open("GET", path, true);

      request.responseType = "document";

      request.onreadystatechange = function() {
        // Request finished.
        if (request.readyState === 4) {
          if (request.status === 200) {
            if (typeof fn_handle_done === "function") {
              fn_handle_done(request);
            }
          } else {
            if (typeof fn_handle_error === "function") {
              fn_handle_error(request);
            }
          }
        }
      };

      request.send();
    }
  };
})();
