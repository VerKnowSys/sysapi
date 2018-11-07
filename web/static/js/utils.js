
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
        $('select.cell_names').append("<option>".concat(cell.name).concat("</option>"));
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
        if (cell != undefined && cell != "") {
          $.ajax({
            type: "GET",
            url: "/snapshot/list/".concat(cell.cell_name),
            dataType: "json",
            contentType : "application/json",
            success: function(data) {
              for (var i = data.list.length - 1; i >= 0; i--) {
                var dataset_and_snapshot = data.list[i];
                if (dataset_and_snapshot != undefined && dataset_and_snapshot != "") {
                  $('select.snapshot_names').append("<option>".concat(dataset_and_snapshot).concat("</option>"));
                } else {
                  $("select#snapshot_name").addClass("is-invalid");
                }
              }
            },
            error: function(doc, err) {
              $("select#snapshot_name").addClass("is-invalid");
            }
          });
        }
      }
    }
  });
}


// Auto-Fill select with available Cell datasets
function fill_list_of_datasets() {
  var selected_cell_name = $("select.cell_name").val();
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
            success: function(data) {
              for (var i = data.list.length - 1; i >= 0; i--) {
                var dataset_and_snapshot = data.list[i];
                if (dataset_and_snapshot != undefined && dataset_and_snapshot != "") {
                  $('select.datasets_names').append("<option>".concat(dataset_and_snapshot).concat("</option>"));
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
