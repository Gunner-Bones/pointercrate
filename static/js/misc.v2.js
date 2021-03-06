function forall(selector, callback) {
  $(selector).each((i, elem) => callback(i, $(elem)));
}

$(window).on("load resize", function() {
  function forceRatio(element, wRatio, hRatio) {
    var target = $(element);
    target.height((target.width() * hRatio) / wRatio);
  }

  // back to top things

  var scrollers = $(".js-scroll");
  var scrollTarget = $("html, body");

  scrollers.each((i, elem) => {
    var src = $(elem);

    src.click(() => {
      var dest = src.data("destination");
      var destination = $("#" + dest);

      if (dest !== undefined)
        scrollTarget.animate({ scrollTop: destination.offset().top - 60 }, 400);
      else scrollTarget.animate({ scrollTop: 0 }, 400);

      if (src.data("reveal")) destination.fadeIn(1000);
    });
  });

  // sometimes animating scrolltop causes things to get stuck. This fixes it.

  $(window).bind("mousewheel touchmove touchstart", function() {
    $("html, body").stop();
  });

  // Closable panels

  forall(".plus.cross", (i, elem) => {
    var parent = elem.parent();

    if (parent.hasClass("closable")) {
      elem.click(() => parent.fadeOut(1000));
    }
  });

  // Animation stuff when scrolling
  var toAnimate = $(".js-scroll-anim");
  var wnd = $(window);

  wnd.on("scroll resize", checkAnimations);
  toAnimate.each((i, elem) => {
    var obj = $(elem);

    if (obj.data("js-shown") !== undefined) return;

    var observer = new MutationObserver(checkAnimations);
    var conf = {
      childList: false,
      attributes: true,
      characterData: false,
      attributeFilter: ["style"],
      subtree: true
    };
    observer.observe(elem.parentElement, conf);

    obj.data("js-shown", true);
  });

  checkAnimations();

  function checkAnimations() {
    var viewportBottom = wnd.scrollTop() + wnd.innerHeight();

    toAnimate.each((i, elem) => {
      var obj = $(elem);
      var objBottom = obj.offset().top;

      if (objBottom <= viewportBottom && !obj.data("js-shown")) {
        switch (obj.data("anim")) {
          default:
          case "fade":
            obj.stop().fadeTo(500, 1);
            break;
        }
        obj.data("js-shown", true);
      } else if (objBottom > viewportBottom && obj.data("js-shown")) {
        switch (obj.data("anim")) {
          default:
          case "fade":
            obj.stop().fadeTo(500, 0);
            break;
        }
        obj.data("js-shown", false);
      }
    });
  }

  // ratio things

  $(".ratio-16-9").each(function() {
    forceRatio(this, 16, 9);
  });
  $(".ratio-4-3").each(function() {
    forceRatio(this, 4, 3);
  });
});
