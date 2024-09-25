A prototype social network application.
# С1. (3 бали) MVC.
>розробити систему, використовуючи MVC-підхід.

[PostgreSQL](https://www.postgresql.org/) + [sqlx](https://docs.rs/sqlx/latest/sqlx/) for models
[Tera](https://keats.github.io/tera/) + [HTMX](https://htmx.org/) for views
[Rocket](https://rocket.rs/) for controllers

## Models
>в моделі 3+ сутностей зі зв’язками
### User
- `id` int
- `nickname` text
~~- `email` text~~
>одне з полів має бути типу файл (посилання)
- `avatar` bytes
- `info` text
- `joined` timestamp
- `phc` bytes
### Post
- `id` int
- `owner_id` int
- `title` text
- `content` text
- `created` timestamp
- `edited` timestamp or Null
- `hidden` timestamp or Null
### Tag
- `post_id` int
- `user_id` int
- `tag` text
- `agree` bool
### Routes
>реалізувати CRUD
#### User CRUD
GET, POST `/login`
GET, POST `/register`
GET `/logout`
GET, PATCH, DELETE `/profile`
GET `/profile/edit`
#### Post CRUD
GET `/@<owner>[#<tag>]*`
GET, PATCH, DELETE `/post/<post_id>`
GET, POST `/post/new`
#### Tag CRUD
PUT, DELETE `/post/<post_id>?tag=<tag>[&agree=<agree>]`
# C19. (3 бали) Статичний контент.
- Використати Blob storage або CDN для статичного контенту (відео, аудіо, зображення, тощо).
[What is a CDN?](https://www.keycdn.com/what-is-a-cdn)
# QA2. (3 бали) E2E тестування.
- Розробити End2End тести для веб додатку з C1, C7, F2, C12 або F5.
- Оцінити покриття.
# QA1. (2 бали) Автоматичне тестування Web API.
- Налаштувати середовище для тестування Web API для проєкту з C5 з використанням можливостей Postman, SOAP UI, або подібного ПЗ.
# С4. (1 бал) Контейнеризація.
Контейнеризувати проєкт у Docker-образ та використати його для розгортання проєкту у середовищі оркестрації контейнерів (Docker Compose, Kubernetes). Розгортання може бути виконано локально.
# C6. (3 бали) Bootstrap (Landing Page).
- Розробити Landing-page об’єктів предметної області, використовуючи Bootstrap
- вся інформація - на одній сторінці зі scroll
- контакти постійно на екрані;
- наприклад, якщо в базі є товари - треба розробити Landing Page для кожного товару, що є в базі, з фото, описом і user-friendly подачею - окремо
- тобто покращити сторінку “Details” в MVC для цієї (обраної) сутності).
# С7. (1 бал) SEO оптимізація сторінки.
- Оптимізувати сторінку з С6 для відображення у пошукових системах.
- Оцінити сайт з використанням безкоштовних сервісів для SEO-аудиту (наприклад, Seobility).
[What is SEO?](https://searchengineland.com/guide/what-is-seo)
# F2. (4 бали) JavaScript + JQuery (+AJAX).
- Autocomplete.
- Drop-Down елемент(и) у формах редагування MVC-проєкту замінити на autocomplete selector із завантаженням з back-end підходящих елементів для випадаючого списку, що містять слова введеного у рядку,
- використати JQuery UI Autocomplete, SELECT2 або інший елемент
- від 3 символів із невеликою затримкою після вводу з клавіатури у це поле
- вибір з великої кількості варіантів – бажано мільйонів+
>як правило, ці параметри конфігуруються у налаштуваннях елементу autocomplete selector.
# C5. (3 бали) JSON/XML API.
- Додати до проекту реалізацію CRUD-функцій через JSON
- GET-запити вертають множину JSON-об’єктів
- POST-запити - з тілом JSON зі змінами - вертають “Ok”/”Error”
- тобто здублювати функціонал з задачі [[#С1. (3 бали) MVC.]] у вигляді REST API
- продемонструвати роботу за допомогою CURL/PostMan/etc.
# B1. (2 бали) Pagination API.
- Додати посторінкову видачу даних для C5.
- Реалізувати skip/limit/nextLink.
- Можна реалізувати протокол OData.
[ReST API Pagination](https://www.merge.dev/blog/rest-api-pagination)
# С16. (3 бали) Автентифікація та авторизація. 
- Реалізувати OAuth2 на основі існуючого провайдера (Microsoft, Google, Facebook).
- Додати можливість реєстрації та входу у додаток (або отримання токену) до С1/С5.
# QA3. (3 бали) Тестування процесу автентифікації/авторизації.
- Розробити набір тестів до процесу автентифікації/авторизації.
# B5. (3 бали) Ізоляція за тенантами.
- Додайте у С16 можливість ізоляції груп користувачів на тенанти.
- Прикладом є будь-який SaaS додаток.
[Explanation by Amazon](https://docs.aws.amazon.com/whitepapers/latest/saas-architecture-fundamentals/tenant-isolation.html)
# B6. (3 бали) Feature flags.
- Додайте у С16 можливість зміни планів користування на основі флагів
- наприклад, користувачі з Premium доступом мають доступ до «особливої» сторінки додатку.
# B4. (3 бали) Пошук.
- Використати ElasticSearch/Lucene/Azure Search (Cognitive Services) для API пошуку (до проєкту С5).
# C3. (2 бали) CI/CD.
- Налаштувати автоматичне розгортання проєкту у хмарному середовищі.
- Зміни в проєкті мають автоматично розгортатися після оновлення main-гілки в системі контролю версій.
- В якості платформи можна використати GitHub Actions, Azure DevOps, Azure WebApp Deployment та інші
# С2. (2 бали) Cloud.
- Викласти проект в Інтернет використовуючи CSP (Azure або інше cloud-сховище: Heroku, DigitalOcean etc.).
- Рекомендовано скористатися @knu.ua адресою для отримання безкоштовного доступу до хмарних ресурсів: a. Azure for Students b. Google Cloud for Students c. AWS Educate
