// Create CELL:
function create_cell() {
    var name = $("input#cell_form_name").val();
    var key = $("textarea#cell_form_key").val();
    if (key != undefined && name != undefined && key != "" && name != "") {
        var url = "/cell/".concat(name);
        $.ajax({
            type: "POST",
            url: url,
            data: key,
            dataType: "json",
            contentType : "application/json",
            statusCode: {
              406: function() { // not allowed
                $("textarea#cell_form_key").addClass("is-invalid");
              }
            },
            success: function(){
                $("input#cell_form_name").removeClass("is-invalid");
                $("textarea#cell_form_key").removeClass("is-invalid");
                $("div.valid-feedback").show();
            }
        });
    } else {
        if (name == "" || name == undefined) {
            $("input#cell_form_name").addClass("is-invalid");
        } else {
            $("input#cell_form_name").removeClass("is-invalid");
            $("input#cell_form_name").addClass("is-valid");
        }
        $("textarea#cell_form_key").addClass("is-invalid");
    }
}


// Delete CELL:
function delete_cell(name) {
  if (name != undefined && name != "") {
      var url = "/cell/".concat(name);
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
            console.log("Success rendering with given name: " + name);
            clean_cells();
            render_cells();
          }
      });
  } else {
      console.log("Ignored invalid DELETE");
  }
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - New Cell");

  // Handle delete cell (once):
  $(document).off("click",".delete_cell");
  $(document).on("click",".delete_cell", function () {
     var clicked_cell_name = $(this).attr('name');
     delete_cell(clicked_cell_name);
  });

});
