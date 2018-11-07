// Create Snapshot:
function create_snapshot() {
    var cell_name = $("select#cell_name").val();
    var snapshot_name = $("input#snapshot_name").val();
    var dataset_path = $("input#snapshot_dataset_path").val();
    if (snapshot_name != undefined && cell_name != undefined && snapshot_name != "" && cell_name != "" && dataset_path != undefined && dataset_path != "") {
        var url = "/snapshot/".concat(cell_name).concat("/").concat(snapshot_name);
        $.ajax({
            type: "POST",
            url: url,
            data: dataset_path,
            dataType: "json",
            contentType : "application/json",
            statusCode: {
              406: function() { // not allowed
                $("input#snapshot_name").addClass("is-invalid");
              }
            },
            success: function(){
                $("select#cell_name").removeClass("is-invalid");
                $("input#snapshot_name").removeClass("is-invalid");
                $("div.valid-feedback").show();
            }
        });
    } else {
        if (cell_name == "" || cell_name == undefined) {
            $("select#cell_name").addClass("is-invalid");
        } else {
            $("select#cell_name").removeClass("is-invalid");
            $("select#cell_name").addClass("is-valid");
        }
        $("input#snapshot_name").addClass("is-invalid");
    }
}


// Delete Snapshot:
function delete_snapshot(cell_name, dataset_path, snapshot_name) {
  if (cell_name != undefined && cell_name != "" && snapshot_name != undefined && snapshot_name != "" && snapshot_name != undefined && snapshot_name != "") {
      var url = "/cell/".concat(cell_name).concat("/").concat(snapshot_name);
      $.ajax({
          type: "DELETE",
          url: url,
          data: dataset_path,
          dataType: "json",
          contentType : "application/json",
          statusCode: {
            304: function() { // not allowed
              console.log("Not modified: " + cell_name);
            }
          },
          success: function(){
            console.log("Destroyed Snapshot: ".concat(dataset_path).concat("@").concat(snapshot_name));
            clean_snapshots();
            render_snapshots();
          }
      });
  } else {
      console.log("Ignored invalid DELETE");
  }
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - New ZFS Snapshot");

  // Handle delete cell (once):
  $(document).off("click",".delete_snapshot");
  $(document).on("click",".delete_snapshot", function () {
     var cell_name = $(this).attr('cell_name');
     var snapshot_name = $(this).attr('snapshot_name');
     var dataset_path = $(this).attr('dataset_path');
     delete_snapshot(cell_name, dataset_path, snapshot_name);
  });

});
