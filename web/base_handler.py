import pdftotext

from http import HTTPStatus
from http.server import BaseHTTPRequestHandler

import cgi, os, re, subprocess
import sys


def default_content_type(_):
    return "text/csv"


def default_filename(_):
    return "scores.csv"


class BaseHandler(BaseHTTPRequestHandler):
    def add_cors_headers(self, origin):
        self.send_header("Access-Control-Allowed-Methods", "POST")
        self.send_header("Access-Control-Allow-Origin", origin)

    def test_origin(self, origin):
        allowed_origins = os.getenv("ALLOWED_ORIGINS")
        if allowed_origins is None or allowed_origins == "*":
            return True
        else:
            allowed_origins = allowed_origins.split(" ")
            for allowed_origin in allowed_origins:
                if re.search(r"{}$".format(allowed_origin), origin):
                    return True

        return False

    def do_OPTIONS(self):
        origin = self.headers.get("origin")
        if self.test_origin(origin):
            self.add_cors_headers(origin)
        else:
            self.add_cors_headers(os.getenv("ALLOWED_ORIGINS"))

        self.send_response(HTTPStatus.OK)
        self.end_headers()

    def handle_POST(
        self,
        bintool,
        get_filename=default_filename,
        get_content_type=default_content_type,
    ):
        origin = self.headers.get("origin")

        if not self.test_origin(origin):
            self.send_error(HTTPStatus.BAD_REQUEST)
            return

        content_length = self.headers.get("content-length")
        if not content_length:
            self.send_error(HTTPStatus.LENGTH_REQUIRED)
            return

        form = cgi.FieldStorage(
            fp=self.rfile,
            headers=self.headers,
            environ={
                "REQUEST_METHOD": "POST",
                "CONTENT_TYPE": self.headers["content-type"],
            },
        )
        raw = "".join(pdftotext.PDF(form["file"].file, raw=True))
        p = subprocess.run(
            [bintool, *form.getlist("options")],
            input=raw,
            text=True,
            capture_output=True,
        )
        self.send_response(HTTPStatus.OK)
        self.send_header("Content-Type", get_content_type(form))
        self.send_header(
            "Content-Disposition",
            'attachment; filename="{}"'.format(get_filename(form)),
        )
        self.end_headers()

        self.wfile.write(bytes(p.stdout, "ascii"))

        return
