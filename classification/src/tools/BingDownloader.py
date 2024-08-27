import os
import shutil
from pathlib import Path
import urllib.request
import urllib
import imghdr
import posixpath

import re
def download(query, limit=100, output_dir='dataset', adult_filter_off=True, 
force_replace=False, timeout=60, filter="", verbose=True):

    # engine = 'bing'
    adult = 'off' if adult_filter_off else 'on'
    image_dir = Path(output_dir).absolute()

    if force_replace and Path.is_dir(image_dir):
        shutil.rmtree(image_dir)

    # check directory and create if necessary
    try:
        if not Path.is_dir(image_dir):
            Path.mkdir(image_dir, parents=True)
    except Exception as e:
        pass

    bing = Bing(query, limit, image_dir, adult, timeout, '', False)
    bing.run()

class Bing:
    def __init__(self, query, limit, output_dir, adult, timeout,  filter='', verbose=True):
        self.download_count = 0
        self.query = query
        self.output_dir = output_dir
        self.adult = adult
        self.filter = filter
        self.verbose = verbose
        self.seen = set()

        assert type(limit) == int, "limit must be integer"
        self.limit = limit
        assert type(timeout) == int, "timeout must be integer"
        self.timeout = timeout

        # self.headers = {'User-Agent': 'Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0'}
        self.page_counter = 0
        self.headers = {'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) ' 
      'AppleWebKit/537.11 (KHTML, like Gecko) '
      'Chrome/23.0.1271.64 Safari/537.11',
      'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
      'Accept-Charset': 'ISO-8859-1,utf-8;q=0.7,*;q=0.3',
      'Accept-Encoding': 'none',
      'Accept-Language': 'en-US,en;q=0.8',
      'Connection': 'keep-alive'}


    def get_filter(self, shorthand):
        if shorthand in ["line", "linedrawing"]:
            return "+filterui:photo-linedrawing"
        elif shorthand == "photo":
            return "+filterui:photo-photo"
        elif shorthand == "clipart":
            return "+filterui:photo-clipart"
        elif shorthand in ["gif", "animatedgif"]:
            return "+filterui:photo-animatedgif"
        elif shorthand == "transparent":
            return "+filterui:photo-transparent"
        else:
            return ""


    def save_image(self, link, file_path):
        request = urllib.request.Request(link, None, self.headers)
        image = urllib.request.urlopen(request, timeout=self.timeout).read()
        if not imghdr.what(None, image):
            raise ValueError('Invalid image, not saving {}\n'.format(link))
        with open(str(file_path), 'wb') as f:
            f.write(image)

    
    def download_image(self, link):
        self.download_count += 1
        try:
            path = urllib.parse.urlsplit(link).path
            filename = posixpath.basename(path).split('?')[0]
            file_type = filename.split(".")[-1]
            if file_type.lower() not in ["jpe", "jpeg", "jfif", "exif", "tiff", "gif", "bmp", "png", "webp", "jpg"]:
                file_type = "jpg"
                
            self.save_image(link, self.output_dir.joinpath('coverart.jpg'))
                        
        except Exception as e:
            self.download_count -= 1

    def run(self):
        total_iter = 0
        while (self.download_count < self.limit) and (total_iter < 10):
            request_url = f'https://www.bing.com/images/async?q={urllib.parse.quote_plus(self.query)}&first={str(self.page_counter)}&count={str(self.limit)}&adlt={self.adult}'

            request = urllib.request.Request(request_url, None, headers=self.headers)
            response = urllib.request.urlopen(request)
            html = response.read().decode('utf8')
            links = re.findall('murl&quot;:&quot;(.*?)&quot;', html)

            for link in links:
                if self.download_count < self.limit:
                    self.download_image(link)
                else:
                    break

            self.page_counter += 1
            total_iter += 1
            