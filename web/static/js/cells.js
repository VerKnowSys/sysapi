

function clean_cells() {
  $("div.cells_list").html("");
}


function render_cells() {
    $.ajax({
        type: "GET",
        url: "/cells/list",
        dataType: "json",
        contentType : "application/json",
        success: function(data){
            for (var i = data.list.length - 1; i >= 0; i--) {
                var cell = data.list[i];
                var cell_template = "<div class=\"col-sm-6 col-lg-4\"> \
<div class=\"brand-card\"> \
  <div class=\"brand-card-header bg-dark\"> \
    <i class=\"fa cui-monitor\"></i> \
  </div> \
  <div class=\"brand-card-body\"> \
      <div class=\"text-value cui-cursor\"></div> \
      <div class=\"text-value cui-moon\"></div> \
      <a href=\"#\" class=\"badge\"><div name=\"__NAME__\" class=\"delete_cell text-value cui-circle-x\"></div></a> \
  </div> \
  <div class=\"brand-card-body\"> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">Name</div> \
      <div class=\"small\">__NAME__</div> \
    </div> \
  </div> \
  <div class=\"brand-card-body\"> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">Domain</div> \
      <div class=\"small\"><a href=\"http://__DOMAIN__/\">__DOMAIN__</a></div> \
    </div> \
  </div> \
  <div class=\"brand-card-body\"> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">Status</div> \
      <div class=\"small\">__STATUS__</div> \
    </div> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">IPv4</div> \
      <div class=\"small\">__IPV4__</div> \
    </div> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">NetID</div> \
      <div class=\"small\">__NETID__</div> \
    </div> \
  </div> \
</div> \
</div>";
                cell_template = cell_template.replace("__IPV4__", cell.ipv4);
                cell_template = cell_template.replace("__NETID__", cell.netid);
                cell_template = cell_template.replace("__STATUS__", cell.status);
                cell_template = cell_template.replace("__NAME__", cell.name);
                cell_template = cell_template.replace("__NAME__", cell.name);
                cell_template = cell_template.replace("__NAME__", cell.name);
                cell_template = cell_template.replace("__DOMAIN__", cell.domain);
                cell_template = cell_template.replace("__DOMAIN__", cell.domain);
                $("div.cells_list").append(cell_template);
            }
            // $("input#cell_form_name").removeClass("is-invalid");
            // $("textarea#cell_form_key").removeClass("is-invalid");
        }
    });
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - Listing all Cells");
  render_cells();
});
