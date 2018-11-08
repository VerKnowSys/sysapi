
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
        if (i == data.list.length - 1) {
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
  $.ajax({
    type: "GET",
    url: "/cells/list",
    dataType: "json",
    contentType : "application/json",
    success: function(data) {
      for (var i = data.list.length - 1; i >= 0; i--) {
        var cell = data.list[i];
        if (cell.name != undefined && cell.name != "") {
          $.ajax({
            type: "GET",
            url: "/snapshot/list/".concat(cell.name),
            dataType: "json",
            contentType : "application/json",
            success: function(snap_data) {
              for (var j = snap_data.list.length - 1; j >= 0; j--) {
                var dataset_and_snapshot = snap_data.list[j];
                console.log("dataset_and_snapshot: ".concat(dataset_and_snapshot));
                if (dataset_and_snapshot != undefined && dataset_and_snapshot != "") {
                  if (j == snap_data.list.length - 1) {
                    $('select.snapshot_names').append("<option disabled selected hidden value=\"\">Pick a Snapshot</option>");
                  } else {
                    $('select.snapshot_names').append("<option>".concat(dataset_and_snapshot).concat("</option>"));
                  }
                } else {
                  $("select.snapshot_names").addClass("is-invalid");
                }
              }
            },
            error: function(doc, err) {
              $("select.snapshot_names").addClass("is-invalid");
            }
          });
        }
      }
    }
  });
}


// Auto-Fill select with available Cell datasets
function fill_list_of_datasets() {
  var selected_cell_name = $("select.cell_names").val();
  console.log("fill_list_of_datasets(): ".concat(selected_cell_name));
  if (selected_cell_name != undefined && selected_cell_name != "") {
    $.ajax({
      type: "GET",
      url: "/datasets/list/".concat(selected_cell_name),
      dataType: "json",
      contentType : "application/json",
      success: function(data) {
        for (var i = data.list.length - 1; i >= 0; i--) {
          var cell = data.list[i];
          $.ajax({
            type: "GET",
            url: "/datasets/".concat(cell.cell_name),
            dataType: "json",
            contentType : "application/json",
            success: function(datasets) {
              for (var j = datasets.list.length - 1; j >= 0; j--) {
                var dataset_and_snapshot = datasets.list[j];
                if (dataset_and_snapshot != undefined && dataset_and_snapshot != "") {
                  if (j == datasets.list.length - 1) {
                    $('select.datasets_names').append("<option disabled selected hidden value=\"\">Pick a Dataset</option>");
                  } else {
                    $('select.datasets_names').append("<option>".concat(dataset_and_snapshot).concat("</option>"));
                  }
                } else {
                  $("select.datasets_names").addClass("is-invalid");
                }
              }
            },
            error: function(doc, err) {
              $("select.datasets_names").addClass("is-invalid");
            }
          });
        }
      }
    });
  }
}
