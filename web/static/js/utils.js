
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
  });
}
