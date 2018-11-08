// Rollback to snapshot:
function create_rollback() {
    var cell_name = $("select.cell_names").val();
    var snapshot_and_dataset = $("select.snapshot_names").val();
    if (snapshot_and_dataset != undefined && snapshot_and_dataset != "") {
        var dataset_path = snapshot_and_dataset.split("@")[0];
        var snapshot_name = snapshot_and_dataset.split("@")[1];
        console.log("create_rollback(): ".concat(dataset_path).concat("@").concat(snapshot_name));

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
                    $("select.snapshot_names").addClass("is-invalid");
                  }
                },
                success: function(){
                    $("select.cell_names").removeClass("is-invalid");
                    $("select.snapshot_names").removeClass("is-invalid");
                    $("div.valid-feedback").show();
                }
            });
        } else {
            if (cell_name == "" || cell_name == undefined) {
                $("select.cell_names").addClass("is-invalid");
            } else {
                $("select.cell_names").removeClass("is-invalid");
                $("select.cell_names").addClass("is-valid");
            }
            $("select.snapshot_names").addClass("is-invalid");
        }
    }
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - New ZFS Rollback");
  fill_list_of_cells();
});
