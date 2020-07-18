from flask import Flask, render_template, request

import subprocess
import threading
import time
import os

import pprint
pp = pprint.PrettyPrinter(indent=4)
import json

app = Flask(__name__)

def get_images_for(uid, sid):
    return []

@app.route('/<uid>/<sid>')
def index(uid, sid):
    return render_template('index.html', uid=uid, sid=sid)

@app.route('/<uid>/<sid>/archive')
def archive(uid, sid):
    imgs = get_images_for(uid, sid)
    return render_template('archive.html', imgs=imgs)

if __name__ == '__main__':
  app.run(host = '0.0.0.0', port = 6060, debug = False)
