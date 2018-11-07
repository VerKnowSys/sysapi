// Create WEB PROXY:
function create_proxy() {
    var cell_name = $("input#proxy_cell_name").val();
    var internal = $("input#proxy_from").val();
    var external = $("input#proxy_to").val();
    if (cell_name != undefined && external != undefined && internal != undefined && external != "" && internal != "" && cell_name != "") {
        var url = "/proxy/".concat(cell_name).concat("/").concat(external).concat("/").concat(internal);
        $.ajax({
            type: "POST",
            url: url,
            dataType: "json",
            contentType : "application/json",
            statusCode: {
              406: function() { // not allowed
                $("input#proxy_to").addClass("is-invalid");
              }
            },
            success: function(){
                $("input#proxy_from").removeClass("is-invalid");
                $("input#proxy_to").removeClass("is-invalid");
                $("div.valid-feedback").show();
            }
        });
    } else {
        if (internal == "" || internal == undefined) {
            $("input#proxy_from").addClass("is-invalid");
        } else {
            $("input#proxy_from").removeClass("is-invalid");
            $("input#proxy_from").addClass("is-valid");
        }
        $("input#proxy_to").addClass("is-invalid");
    }
}


// Delete WEB PROXY:
function delete_proxy(name, internal, external) {
  if (name != undefined && name != "") {
      var url = "/proxy/".concat(name).concat("/").concat(external).concat("/").concat(internal);
      $.ajax({
          type: "DELETE",
          url: url,
          dataType: "json",
          contentType : "application/json",
          statusCode: {
            304: function() { // not allowed
              console.log("Not modified: " + name);
            }
          },
          success: function(){
            console.log("Success rendering proxy with given name: " + name);
            clean_proxies();
            render_proxies();
          }
      });
  } else {
      console.log("Ignored invalid DELETE");
  }
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - New Proxy");

  // Handle delete cell (once):
  $(document).off("click",".delete_proxy");
  $(document).on("click",".delete_proxy", function () {
     var cell_name = $(this).attr('name');
     var proxy_from = $(this).attr('from');
     var proxy_to = $(this).attr('to');
     delete_proxy(cell_name, proxy_from, proxy_to);
  });

});
