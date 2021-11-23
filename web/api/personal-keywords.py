from base_handler import BaseHandler


def get_filename(form):
    options = form.getlist("options")
    if "--by-topic" in options and "--list" in options:
        return "keywords.md"

    return "keywords.csv"


def get_content_type(form):
    options = form.getlist("options")
    if "--by-topic" in options and "--list" in options:
        return "text/markdown"

    return "text/csv"


class handler(BaseHandler):
    def do_POST(self):
        self.handle_POST("./bin/ite-personal-keywords", get_filename, get_content_type)
