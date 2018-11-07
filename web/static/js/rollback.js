// Rollback to snapshot:
function create_rollback() {
    var cell_name = $("select#cell_name").val();
    var snapshot_name = $("input#snapshot_name").val();
    var dataset_path = $("input#snapshot_dataset_path").val();
    if (snapshot_name != undefined && cell_name != undefined && snapshot_name != "" && cell_name != "" && dataset_path != undefined && dataset_path != "") {
        var url = "/rollback/".concat(cell_name).concat("/").concat(snapshot_name);
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


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - New ZFS Rollback");
  fill_list_of_cells();
});
