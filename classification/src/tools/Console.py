from time import perf_counter

class Console:
    tic = perf_counter()
    
    def __init__(self):
        self.infoMethod = None
        self.ping = perf_counter()
        
    def toc(self):
        return round((perf_counter() - self.tic) * 60 * 1000)
    
    def pong(self):
        pong = round((perf_counter() - self.ping) * 60 * 1000)
        self.ping = perf_counter()
        return pong
    
    def debug(self, *messages, nl = ''):
        if(nl == True):
            nl = '\n'
        msg = f'[{self.toc()}] [{self.pong()}] {", ".join(messages)}'
        print(f'{nl}[{self.colored(150,255,150, "DEBUG")}]: {msg}')
    pass

    def info(self, *messages):
        msg = f'[{self.toc()}] {", ".join(messages)}'
        if self.infoMethod:
            self.infoMethod.setText(", ".join(messages))
        print(f'[{self.colored(150,150,255, "INFO")}]: {msg}')
    pass

    def link(self, method):
        self.infoMethod = method 

    def error(self, *messages):
        msg = f'[{self.toc()}] {", ".join(messages)}'
        print(f'[{self.colored(255,150,150, "ERROR")}]: {msg}')
    pass

    def colored(self, r, g, b, text):
        return "\033[38;2;{};{};{}m{}\033[38;2;255;255;255m".format(r, g, b, text)


console = Console()
