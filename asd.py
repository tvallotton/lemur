import re
import requests
import datetime
import json
import time



def main():
    def to_dict(text):
        return dict(re.findall("([^:\n]+): ([^\n:]+)", text))

    headers = to_dict('''Host: stackoverflow.com
    User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:83.0) Gecko/20100101 Firefox/83.0
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
    Accept-Language: en-US,en;q=0.5
    Accept-Encoding: gzip, deflate, br
    Referer: https://stackoverflow.com/questions/tagged/f%23
    Connection: keep-alive
    Cookie: prov=c1184384-696c-8309-3761-0cb5f6ea33ec; _ga=GA1.2.190517247.1603570192; __qca=P0-1346126975-1603570195485; acct=t=VgZIwXCaMV9EKmUtah%2bD6GLn2AR%2bdyW%2b&s=%2fyEyBRJW1kCOoZAKHmOECe5DG4qnlFjd; __gads=ID=f8b6cb55935c0dab-225ac71d44b80010:T=1603570201:S=ALNI_Max82VwboMK5-bf_bk1FZ3upT5Idg; sgt=id=9259061f-c269-4d95-bbc1-aeefea369221; _gid=GA1.2.784542305.1608042205; _gat=1
    Upgrade-Insecure-Requests: 1
    Cache-Control: max-age=0
    TE: Trailers''')


    languages = ['python',
                 'rust',
                 'lua',
                 'javascript',
                 'haskell',
                 'go',
                 'swift',
                 'c++',
                 'c#',
                 'c',
                 'objective-c',
                 'java', 'php',
                 'perl',
                 'fortran',
                 'common-lisp',
                 'clojure',
                 'scheme',
                 'f%23',
                 'elm',
                 'erlang',
                 'r',
                 'ruby',
                 'scala',
                 'kotlin',
                 'typescript',
                 'dart',
                 'bash']




    base_url = 'https://stackoverflow.com/questions/tagged/{}'
    urls = [base_url.format(lang) for lang in languages]

    regex = '([\d,]+)[\n\r\t]+questions'

    questions = {}
    for url, lang in zip(urls, languages):
        if lang in questions.keys():
            continue
        r = requests.get(url)
        num = re.findall(regex, r.text)[0]
        num = re.sub(',', '', num)
        num = int(num)
        questions[lang] = num
        print(lang)
        time.sleep(2)


    today = datetime.date.today()


    todays_data = {'date': {'day': today.day, 'month': today.month, 'year': today.year},
    'questions_per_language': questions}


    with open('data.json') as f:
        data = json.load(f)



    data['data'].append(todays_data)


    data

    with open('data.json', 'w') as f:
        json.dump(data, f)




while True:
    main()
    time.sleep(24 * 60 * 60)
