from base_handler import BaseHandler


class handler(BaseHandler):
    def do_POST(self):
        self.handle_POST("./bin/ite-personal-scores")
