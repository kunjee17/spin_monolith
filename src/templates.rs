use tera::Tera;
use anyhow::Result;

pub fn get_tera() -> Result<Tera> {
    let mut tera = Tera::default();
    tera.add_raw_templates(vec![
    (
        "base",
        "<!doctypehtml><html lang=en><meta charset=utf-8><meta content=width=device-width,initial-scale=1 name=viewport><link crossorigin href=https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css integrity=sha384-QWTKZyjpPEjISv5WaRU9OFeRpok6YctnYmDr5pNlyT2bRjXh0JMhjY6hW+ALEwIH rel=stylesheet><title>{% block title %}{% endblock title %} - My Webpage</title> {% block head %} {% endblock head %}<body><div class=container-fluid id=content>{% block content %}{% endblock content %}</div><footer class=\"mt-auto bg-light text-center py-3\"><script crossorigin integrity=sha256-/JqT3SQfawRcv/BIHPThkBvs0OEvtFFmqPF/lYI/Cxo= src=https://code.jquery.com/jquery-3.7.1.min.js></script><script crossorigin integrity=sha384-YvpcrYf0tY3lHB60NNkmXc5s9fDVZLESaAA55NDzOxhy9GkcIdslK1eN7N6jIeHz src=https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.bundle.min.js></script> {% block footer %}{% endblock footer %} © Copyright {{ now() | date(format=\"%Y\") }} by <a href=http://domain.invalid/>you</a>.</footer>",
    ),
    (
        "index",
        "{% extends \"base\" %} {% block head %} {{ super() }} {% endblock head %} {% block title %}Index{% endblock title %} {% block content %} <div class=mt-5><h1>Login</h1><form action=/login id=login-form method=POST><div class=mb-3><label class=form-label for=email>Email address</label><input placeholder=\"Enter email\"class=form-control id=email name=email required type=email></div><div class=mb-3><label class=form-label for=password>Password</label><input class=form-control id=password name=password placeholder=Password required type=password></div><button class=\"btn btn-primary\">Login</button><a class=\"btn btn-link\"href=/register>Register</a></form></div> {% endblock content %} {% block footer %} {{ super() }} <script>$(document).ready(function() {\r\n        $('#login-form').submit(function(event) {\r\n            const email = $('#email').val();\r\n            const password = $('#password').val();\r\n            if (email === '' || password === '') {\r\n                alert('Both fields are required.');\r\n                event.preventDefault();\r\n            }\r\n        });\r\n    });</script> {% endblock footer %}",
    ),
    (
        "product",
        "{% extends \"base\" %} {% block title %}Index{% endblock title %} {% block head %} {{ super() }} {% endblock head %} {% block content %} <h1>Products</h1><table class=\"table table-striped\"><thead><tr><th>ID<th>Name<th>Description<th>Price<tbody>{% for product in products %} <tr><td>{{ product.id }}<td>{{ product.name }}<td>{{ product.description }}<td>{{ product.price }}</tr> {% endfor %}</table> {% endblock content %}",
    ),
    (
        "register",
        "{% extends \"base\" %} {% block head %} {{ super() }} {% endblock head %} {% block title %}Register{% endblock title %} {% block content %} <div class=mt-5><h1>Register</h1><form action=/register id=register-form method=POST><div class=mb-3><label class=form-label for=email>Email address</label><input placeholder=\"Enter email\"class=form-control id=email name=email required type=email></div><div class=mb-3><label class=form-label for=password>Password</label><input class=form-control id=password name=password placeholder=Password required type=password></div><div class=mb-3><label class=form-label for=confirm-password>Confirm Password</label><input placeholder=\"Confirm Password\"class=form-control id=confirm-password name=confirm_password required type=password></div><button class=\"btn btn-primary\">Register</button><a class=\"btn btn-link\"href=/>Login</a></form></div> {% endblock content %} {% block footer %} {{ super() }} <script>$(document).ready(function() {\r\n        $('#register-form').submit(function(event) {\r\n            const email = $('#email').val();\r\n            const password = $('#password').val();\r\n            const confirmPassword = $('#confirm-password').val();\r\n            if (email === '' || password === '' || confirmPassword === '') {\r\n                alert('All fields are required.');\r\n                event.preventDefault();\r\n            } else if (password !== confirmPassword) {\r\n                alert('Passwords do not match.');\r\n                event.preventDefault();\r\n            }\r\n        });\r\n    });</script> {% endblock footer %}",
    ),
])?;
    Ok(tera)
}
