

function clean_proxies() {
  $("div.proxies_list").html("");
}


function render_proxies() {
    $.ajax({
        type: "GET",
        url: "/proxies/list",
        dataType: "json",
        contentType : "application/json",
        success: function(data){
            for (var i = data.list.length - 1; i >= 0; i--) {
                var proxy = data.list[i];
                var proxy_template = "<div class=\"col-sm-6 col-lg-4\"> \
<div class=\"brand-card\"> \
  <div class=\"brand-card-header bg-dark\"> \
    <i class=\"fa cui-monitor\"></i> \
  </div> \
  <div class=\"brand-card-body\"> \
      <div class=\"text-value\"></div> \
      <a href=\"#\" class=\"badge\"><div name=\"__NAME__\" from=\"__FROM__\" to=\"__TO__\" class=\"delete_proxy text-value cui-circle-x\"></div></a> \
  </div> \
  <div class=\"brand-card-body\"> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">Cell</div> \
      <div class=\"small\">__NAME__</div> \
    </div> \
  </div> \
  <div class=\"brand-card-body\"> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">From</div> \
      <div class=\"small\"><a href=\"http://__FROM__/\">__FROM__</a></div> \
      <div class=\"small\">__IPFROM__</div> \
    </div> \
  </div> \
  <div class=\"brand-card-body\"> \
    <div> \
      <div class=\"text-value text-muted text-uppercase\">To</div> \
      <div class=\"small\"><a href=\"http://__TO__/\">__TO__</a></div> \
      <div class=\"small\">__IPTO__</div> \
    </div> \
  </div> \
</div> \
</div>";
                proxy_template = proxy_template.replace(/__NAME__/g, proxy.cell);
                proxy_template = proxy_template.replace(/__FROM__/g, proxy.from);
                proxy_template = proxy_template.replace(/__IPFROM__/g, proxy.from_ipv4);
                proxy_template = proxy_template.replace(/__IPTO__/g, proxy.to_ipv4);
                proxy_template = proxy_template.replace(/__TO__/g, proxy.to);
                $("div.proxies_list").append(proxy_template);
            }
            // $("input#proxy_form_name").removeClass("is-invalid");
            // $("textarea#proxy_form_key").removeClass("is-invalid");
        }
    });
}


$( document ).ready(function() {
  $('li.location_info').text("System Management Interface - Listing all Proxies");
  render_proxies();
});
