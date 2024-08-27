class ColorTheme:
    def __init__(self):
        self.loadStyleFile()
        
    def loadStyleFile(self):
        with open('./styles/style.css', 'r') as file:
            self.data = file.read().replace('\n', ' ')
            
    def loadData(self):
        color_pallete = {
            'text': "#1e2129",
            'text-error': '#f55951',
            'text-info': '#edd2cb',
            
            'background': "#20131e",
            'background-toolbar': "#543c52",
            'background-player': "#edd2cb",
            
            'button-enabled': '#EEF5DB',
            'button-hover': '#f9fcf4',
            'button-pressed': '#f6faed',
            'button-disabled': '#474941',
            
            'slider-post':'#C7EFCF',
            'slider-pre': '#8ba790',
            
            'progress-done': '#232630',
            'progress-load': '#191b22',
        }
        
        css_string = self.data

        for color in color_pallete:
            css_string = css_string.replace(f'color({color})', color_pallete[color])
        return css_string