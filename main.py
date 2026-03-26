import requests
from bs4 import BeautifulSoup
from urllib.parse import urljoin
from http.cookies import SimpleCookie
import os
import sys

base_url = 'https://isis.verw.uni-koeln.de'
cgi_url = f"{base_url}/cgi/self-service.cgi"
klident = os.environ["UNISPORT_KLIDENT"]
klcode = os.environ["UNISPORT_KLCODE"]
password = os.environ["UNISPORT_PASSWORD"]

def get_login_data():
    login_page_url = f'{cgi_url}?klident={klident}&klcode={klcode}'
    login_page_response = requests.get(login_page_url)
    html = BeautifulSoup(login_page_response.text, features="html.parser")
    form = html.select_one("form")
    data = {
            "ss_passwort": password,
            "klcode": form.select_one('input[name="klcode"]').get("value"),
            "klident": form.select_one('input[name="klident"]').get("value")
            }
    return data

def get_kurscode(kursnr, login_data):
    courses_page_response = requests.post(cgi_url, login_data)
    courses_page_html = BeautifulSoup(courses_page_response.text, features="html.parser")

    kursnr_tds = courses_page_html.select(".td_kursnr")
    kurscode = None
    for kursnr_td in kursnr_tds:
        if kursnr_td.text == str(kursnr):
            kurscode = kursnr_td.parent.get("data-kurscode")
            break
    if kurscode is None:
        raise RuntimeError("kurscode not found")

    cookie = SimpleCookie(courses_page_response.headers['Set-Cookie'])
    bs_sspw = {key: value.value  for key, value in cookie.items()}['bs_sspw']

    return kurscode, f"bs_sspw={bs_sspw}"

def get_course_pdf_location(kurscode, login_data, cookie):
    course_print_pdf_url = f'{cgi_url}?klident={login_data["klident"]}&klcode={login_data["klcode"]}&kurscode={kurscode}&action=print'
    course_pdf_response = requests.get(course_print_pdf_url, headers={'Cookie': cookie}, allow_redirects=False)
    return course_pdf_response.headers['Location']

def get_course_pdf(location, cookie):
    pdf_url = f"{base_url}{location}"
    return requests.get(pdf_url, headers={'Cookie': cookie})

if __name__ == '__main__':
    kursnr = sys.argv[1]
    login_data = get_login_data()
    kurscode, cookie = get_kurscode(kursnr, login_data)
    course_pdf_location = get_course_pdf_location(kurscode, login_data, cookie)
    course_pdf = get_course_pdf(course_pdf_location, cookie)
    with open(f"participants_{kursnr}.pdf", mode="wb") as io:
        io.write(course_pdf.content)

