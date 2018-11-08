

function clean_snapshots() {
  $("tbody.snapshots_list").html("");
}


function render_snapshots() {
  var cell_name = $("select#cell_name").val();
  if (cell_name != null && cell_name != undefined && cell_name != "") {
    $.ajax({
      type: "GET",
      url: "/snapshot/list/".concat(cell_name),
      dataType: "json",
      contentType : "application/json",
      success: function(snaps) {
        console.log("Got: " + snaps.list.length + " snapshots.");
        for (var j = snaps.list.length - 1; j >= 0; j--) {
          var dataset_and_snap = snaps.list[j];
          var snapshot_name = dataset_and_snap.split("@")[1];
          if (snapshot_name == "origin" || snapshot_name == "after_export")
            continue;

          console.log("Got: '" + dataset_and_snap + "' - 'dataset@snapshot'");
          $.ajax({
            type: "GET",
            url: "/snapshot/".concat(cell_name).concat("/").concat(snapshot_name),
            dataType: "json",
            contentType : "application/json",
            success: function(snapshot_obj) {
              console.log("SNAPSHOT_OBJECT: " + JSON.stringify(snapshot_obj));
              var snapshot_template = "\
<tr class=\"delete_snapshot\" cell_name=\"__CELL_NAME__\" snapshot_name=\"__SNAPSHOT_NAME__\" dataset_path=\"__DATASET_PATH__\"> \
  <td>__CELL_NAME__</td> \
  <td>__SNAPSHOT_NAME__</td> \
  <td>__DATASET_PATH__</td> \
  <td>__TIMESTAMP__</td> \
</tr> \
";
              snapshot_template = snapshot_template.replace(/__SNAPSHOT_NAME__/g, snapshot_obj.name);
              snapshot_template = snapshot_template.replace(/__CELL_NAME__/g, snapshot_obj.cell_name);
              snapshot_template = snapshot_template.replace(/__DATASET_PATH__/g, snapshot_obj.dataset_path);
              snapshot_template = snapshot_template.replace(/__TIMESTAMP__/g, snapshot_obj.timestamp);
              console.log("Snapshot template: " + snapshot_template);
              $("tbody.snapshots_list").append(snapshot_template);
            }
          });
        }
      }
    });
  }
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - Listing all ZFS Snapshots of all Cells");
  render_snapshots();
});
