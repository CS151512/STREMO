# **STREMO**

<div align="center">

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Status](https://img.shields.io/badge/Status-Active-success.svg)

<br>

![C++](https://img.shields.io/badge/C++-00599C?style=flat&logo=c%2B%2B&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat&logo=typescript&logoColor=white)
![Next.js](https://img.shields.io/badge/Next.js-000000?style=flat&logo=nextdotjs&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat&logo=docker&logoColor=white)
![K3s](https://img.shields.io/badge/K3s-FFC61C?style=flat&logo=k3s&logoColor=black)
![Terraform](https://img.shields.io/badge/Terraform-844FBA?style=flat&logo=terraform&logoColor=white)
![Apache Kafka](https://img.shields.io/badge/Kafka-231F20?style=flat&logo=apachekafka&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-4169E1?style=flat&logo=postgresql&logoColor=white)
![Redis](https://img.shields.io/badge/Redis-DC382D?style=flat&logo=redis&logoColor=white)
![Git](https://img.shields.io/badge/Git-F05032?style=flat&logo=git&logoColor=white)

</div>

![title_img](./img/title-readme.png)

>[!IMPORTANT]
> Данные проект был разработан в рамках дисциплины "ЯПиСД" в виде расчетно графической работы первого курса ФПМИ \
> STREMO — это амбициозный стартап и интерактивная стриминговая платформа для геймеров, креаторов и их аудитории. Зародившись как смелый студенческий проект, STREMO стремится стать площадкой с самым отзывчивым и сплоченным комьюнити. Уже на этапе открытого тестирования STREMO объединяет десятки талантливых стримеров и тысячи зрителей. Наша серверная инфраструктура, обеспечивающая трансляции с минимальной задержкой, постоянно масштабируется. В этом году мы запустили полноценный функционал для создателей контента и предлагаем пользователям удобный инструментарий для проведения прямых эфиров.


## Быстрый запуск

**Git Clone**

```bash
git clone https://github.com/CS151512/STREMO.git
cd STREMO
```

> [!IMPORTANT]
> Полная документация есть в папке docs, а также в makefile есть `make help`, которая описывает весь список команд


## Документация

Проект обладает разветвленной архитектурой, поэтому документация разбита на специализированные разделы:

<table width="100%">
  <tr>
    <td width="50%" valign="top">
      <a href="./docs/Archicture.md">
        <img src="https://img.shields.io/badge/Архитектура_и_Дизайн-2C3E50?style=for-the-badge&logo=c%2B%2B&logoColor=white" alt="Архитектура">
      </a><br><br>
      Верхнеуровневое устройство системы, взаимодействие C++ ядра и брокера Kafka.
    </td>
    <td width="50%" valign="top">
      <a href="./docs/API.md">
        <img src="https://img.shields.io/badge/API_Reference-2C3E50?style=for-the-badge&logo=postman&logoColor=white" alt="API">
      </a><br><br>
      Спецификации контрактов, описание эндпоинтов и форматов передачи данных.
    </td>
  </tr>
  <tr>
    <td width="50%" valign="top">
      <a href="./docs/DataBase.md">
        <img src="https://img.shields.io/badge/База_Данных-2C3E50?style=for-the-badge&logo=postgresql&logoColor=white" alt="База данных">
      </a><br><br>
      Схемы таблиц PostgreSQL, механизмы кэширования в Redis и модели данных.
    </td>
    <td width="50%" valign="top">
      <a href="./docs/Sharding.md">
        <img src="https://img.shields.io/badge/Шардирование-2C3E50?style=for-the-badge&logo=apachekafka&logoColor=white" alt="Шардирование">
      </a><br><br>
      Стратегии распределения данных, партицирование и масштабирование хранилища.
    </td>
  </tr>
  <tr>
    <td width="50%" valign="top">
      <a href="./docs/Deploy.md">
        <img src="https://img.shields.io/badge/Развертывание-2C3E50?style=for-the-badge&logo=terraform&logoColor=white" alt="Развертывание">
      </a><br><br>
      Инструкции по сборке, локальному запуску и настройке k3s кластера с Terraform.
    </td>
    <td width="50%" valign="top">
      <a href="./docs/CI-CD.md">
        <img src="https://img.shields.io/badge/CI%2FCD-2C3E50?style=for-the-badge&logo=githubactions&logoColor=white" alt="CI/CD">
      </a><br><br>
      Пайплайны автоматического тестирования, сборки образов и доставки кода.
    </td>
  </tr>
  <tr>
    <td width="50%" valign="top">
      <a href="./docs/srs/">
        <img src="https://img.shields.io/badge/Мат_модели_(SRS)-2C3E50?style=for-the-badge&logo=latex&logoColor=white" alt="Математика">
      </a><br><br>
      Расчетно-графическая часть: спецификации и формулы (доступны в формате PDF и TeX).
    </td>
    <td width="50%" valign="top">
      <a href="./docs/User.md">
        <img src="https://img.shields.io/badge/Руководство-2C3E50?style=for-the-badge&logo=readthedocs&logoColor=white" alt="Пользователям">
      </a><br><br>
      Описание клиентской части, ролей пользователей и базовых сценариев использования.
    </td>
  </tr>
</table>


## Математические модели и спецификаци

В данном разделе представлено формальное математическое описание работы алгоритмов обработки данных, оценка асимптотической сложности и спецификации структур данных для проекта **STREMO**.

<div align="center">
  <table>
    <tr>
      <td align="center" width="50%">
        <br>
        <a href="./STREMO_Math.pdf">
          <img src="https://img.shields.io/badge/Отчет_в_формате_PDF-E34F26?style=for-the-badge&logo=pdf&logoColor=white" alt="PDF">
        </a>
        <br><br>
        <i>(Рекомендуется для проверки)</i>
      </td>
      <td align="center" width="50%">
        <br>
        <a href="./main.tex">
          <img src="https://img.shields.io/badge/Исходный_код_LaTeX-008080?style=for-the-badge&logo=latex&logoColor=white" alt="LaTeX">
        </a>
        <br><br>
        <i>(Директория с .tex файлами)</i>
      </td>
    </tr>
  </table>
</div>

---

## Самостоятельная сборка из исходников

Если вы хотите скомпилировать PDF-документ локально, убедитесь, что у вас установлен дистрибутив TeX (например, TeX Live или MiKTeX) и выполните следующую команду в этой директории:

```bash
pdflatex main.tex
# Рекомендуется запустить дважды для корректной сборки оглавления и ссылок
pdflatex main.tex
```

---
**by finnik & s1gmagor**
