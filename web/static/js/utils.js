
// Auto-Fill select with available Cell names
function fill_list_of_cells() {
  $.ajax({
    type: "GET",
    url: "/cells/list",
    dataType: "json",
    contentType : "application/json",
    success: function(data) {
      for (var i = data.list.length - 1; i >= 0; i--) {
        var cell = data.list[i];
        if (i == 0) {
          $('select.cell_names').append("<option selected value=\"\">Pick a Cell</option>");
        } else {
          $('select.cell_names').append("<option>".concat(cell.name).concat("</option>"));
        }
      }
    }
  });
}


// Auto-Fill select with available Cell snapshots
function fill_list_of_snapshots() {
  var selected_cell_name = $("select.cell_names").val();
  if (selected_cell_name != undefined && selected_cell_name != "") {
    $.ajax({
      type: "GET",
      url: "/snapshot/list/".concat(selected_cell_name),
      dataType: "json",
      contentType : "application/json",
      success: function(data) {
        for (var j = data.list.length - 1; j >= 0; j--) {
          var full_snapshot_path = data.list[j];
          if (full_snapshot_path != undefined && full_snapshot_path != "") {
            if (j == 0) {
              $('select.snapshot_names').append("<option disabled selected hidden value=\"\">Pick a Snapshot</option>");
            } else {
              $('select.snapshot_names').append("<option>".concat(full_snapshot_path).concat("</option>"));
            }
            $("select.cell_names").removeClass("is-invalid");
            $("select.cell_names").addClass("is-valid");
            $("select.snapshot_names").removeClass("is-invalid");
            $("select.snapshot_names").addClass("is-valid");
          } else {
            $("select.snapshot_names").removeClass("is-valid");
            $("select.snapshot_names").addClass("is-invalid");
          }
        }
      }
      // error: function(doc, err) {
      //   $("select.snapshot_names").addClass("is-invalid");
      // }
    });
  }


}


// Auto-Fill select with available Cell datasets
function fill_list_of_datasets() {
  var selected_cell_name = $("select.cell_names").val();
  if (selected_cell_name != undefined && selected_cell_name != "") {
    $.ajax({
      type: "GET",
      url: "/datasets/list/".concat(selected_cell_name),
      dataType: "json",
      contentType : "application/json",
      success: function(data) {
        for (var i = data.list.length - 1; i >= 0; i--) {
          var dataset = data.list[i];
          if (dataset != undefined && dataset != "") {
            if (i == 0) {
              $('select.datasets_names').append("<option disabled selected hidden value=\"\">Pick a Dataset</option>");
            } else {
              $('select.datasets_names').append("<option>".concat(dataset).concat("</option>"));
            }
          }
        }
      }
    });
  }
}
