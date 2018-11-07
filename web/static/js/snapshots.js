

function clean_snapshots() {
  $("tbody.snapshots_list").html("");
}


function render_snapshots() {
  $.ajax({
      type: "GET",
      url: "/cells/list",
      dataType: "json",
      contentType : "application/json",
      success: function(data){
        for (var i = data.list.length - 1; i >= 0; i--) {
          var cell_name = data.list[i].name;
          if (cell_name != undefined && cell_name != "") {
            $.ajax({
              type: "GET",
              url: "/snapshot/list/".concat(cell_name),
              dataType: "json",
              contentType : "application/json",
              success: function(data){
                  for (var i = data.list.length - 1; i >= 0; i--) {
                      var snapshot = data.list[i];
                      var snapshot_template = "\
  <tr class=\"delete_snapshot\" cell_name=\"__CELL_NAME__\" snapshot_name=\"__SNAPSHOT_NAME__\" dataset_path=\"__DATASET_PATH__\"> \
    <td>__CELL_NAME__</td> \
    <td>__SNAPSHOT_NAME__</td> \
    <td>__DATASET_PATH__</td> \
    <td>__TIMESTAMP__</td> \
  </tr> \
                ";
                      snapshot_template = snapshot_template.replace(/__CELL_NAME__/g, snapshot.cell_name);
                      snapshot_template = snapshot_template.replace(/__DATASET_PATH__/g, snapshot.dataset_path);
                      snapshot_template = snapshot_template.replace(/__SNAPSHOT_NAME__/g, snapshot.name);
                      snapshot_template = snapshot_template.replace(/__TIMESTAMP__/g, snapshot.timestamp);
                      console.log("GNE: ".concat(snapshot_template));
                      $("tbody.snapshots_list").append(snapshot_template);
                  }
              }
            });
          }
        }
      }
    });
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - Listing all ZFS Snapshots of all Cells");
  render_snapshots();
});
