# Рекомендации по взаимодействию с налоговой

## 3-НДФЛ

<a name="tax-statement"></a>
### Формирование налоговой декларации

Investments позволяет в полностью автоматическом режиме производить заполнение 3-НДФЛ в формате программы
[Декларация](https://www.nalog.ru/rn77/program/5961249/) (`*.dcX`).

Чтобы получить заполненную декларацию, необходимо:
1. Создать в программе Декларация налоговую декларацию и сохранить ее на диск. Никакие поля можно не заполнять,
но необходимо отметить галочку `Задание условий -> Имеются доходы -> В иностранной валюте`:
![Создание файла декларации](images/empty-statement.png?raw=true)
2. Далее — запустить `investments tax-statement ib 2020 statement.dc0` указав год (2020) и путь к сохраненному файлу.

Investments внесет все полученные доходы в указанный файл, а также выведет на stdout таблицы расчета, которые
впоследствии можно будет использовать для объяснения полученных цифр инспектору.

Открыв файл снова в программе Декларация, увидим на соответствующей вкладке задекларированные доходы:
![Заполненный файл декларации](images/filled-statement.png?raw=true)

### Что стоит иметь в виду

<a name="dividend-reclassifications"></a>
#### Перерасчет налогов

В начале года (Interactive Brokers - конец февраля, Firstrade - март) зарубежные брокеры производят перерасчет налогов,
уплаченных в прошлом году с дивидендов, возвращая часть удержанных налогов. Причина этого (поддержка IB):
> Every year IB has to adjust the 1042 withholding (i.e. withholding on US dividends paid to non-US accounts) to reflect
> dividend reclassifications. This is typically done in February the following year. As such, the majority of these
> adjustments are refunds to customers. The typical case is when IB's best information at the time of paying a dividend
> indicates that the distribution is an ordinary dividend (and therefore subject to withholding), then later at year end,
> the dividend is reclassified as Return of Capital, proceeds, or capital gains (all of which are not subject to 1042
> withholding).

В результате у нас получается [не очень понятная ситуация](https://www.banki.ru/forum/?PAGE_NAME=message&FID=21&TID=377500&MID=8558603#message8558603):
1. Такого понятия как "возврат уплаченного налога" в 3-НДФЛ нет.
2. Для инспектора такие возвраты выглядят как доход следующего года.
3. Нет корректного способа заплатить 100% налога с такого "дохода" в следующем году.
4. Ко всему прочему, все это еще осложняется тем, что в IB данные возвраты в отчетах имеют даты исходной уплаты налога —
т. е. совершенно не соответствуют дате поступления средств на счет.
   
В целом, если обсуждать этот вопрос на профильных ресурсах, то абсолютное большинство (и я в их числе) сходится в том,
чтобы считать эти возвраты именно возвратами, а именно — уменьшать за их счет величину уплаченного налога. Хотя, есть и
[прецеденты](https://www.banki.ru/forum/?PAGE_NAME=message&FID=21&TID=379285&MID=8493535#message8493535) когда налоговый
инспектор отказывался принимать такую декларацию.

Один из аргументов в пользу данной позиции — пересчитанные налоги будут полностью согласовываться с цифрами в 1042-S.

Учитывая все вышесказанное, Investments при выполнении расчета выполняет пересчет уплаченных налогов, из-за чего
налоговую декларацию имеет смысл формировать не раньше марта в случае IB и не раньше апреля в случае Firstrade.

#### Страна дохода

На данный момент программа все доходы декларирует со страной дохода — США.

<a name="ib-trade-settle-date"></a>
#### Interactive Brokers

С точки зрения налогообложения, все расчеты необходимо проводить на дату расчета сделки, но в Activity Statement
присутствуют только даты заключения сделок. В принципе, можно сформировать налоговую декларацию и по ним — программа
это позволяет сделать, хотя и пишет при этом warning.

Чтобы программа могла рассчитывать сделки на дату расчета, необходимо к Activity Statement [добавить Trade Confirmation
Report](https://github.com/KonishchevDmitry/investments/blob/master/docs/brokers.md#ib-trade-settle-date).

Делать это или нет — целиком ваше решение. В целом, это более корректный способ расчета с точки зрения налогового
законодательства. Из минусов — потенциально больше подтверждающих документов и разъяснений для налоговой инспекции.

### Подтверждающие документы

По моим наблюдениям, большинство людей при отправке декларации пытается приложить к ней как можно больше подтверждающих
документов. Намерение, безусловно, хорошее, но не всегда просто осуществимое — попробуйте, к примеру, перевести на
русский язык (а по закону — все подтверждающие документы должны быть с переводом) отчеты Firstrade — сразу же возникнет
мысль "а не проще ли вообще закрыть этот счет и пользоваться только IB / российскими брокерами". :)

К тому же, в отчетах может быть довольно большое количество различных неочевидных моментов: [перерасчет
налогов](#dividend-reclassifications), [несовпадение
дат в записях о возврате](https://github.com/KonishchevDmitry/investments/blob/master/docs/brokers.md#ib-tax-remapping),
[отсутствие даты расчета сделки в отчетах](#ib-trade-settle-date), корпоративные действия и [корпоративные действия, не
отраженные в отчетах](https://github.com/KonishchevDmitry/investments/issues/29). Если налоговый инспектор действительно
решит взглянуть в ваши отчеты, а не просто не глядя приложить к декларации, то с довольно большой вероятностью у него
возникнут к вам вопросы, откуда взялись те или иные цифры.

Поэтому лично мой подход следующий: по закону у нас есть обязанность задекларировать полученные доходы и уплатить с них
налог, но при этом нет изначальной обязанности тут же прикладывать все возможные доказательства получения этого дохода.
Поэтому мы формируем максимально корректную (средствами программы, что исключает возможность случайных ошибок в
расчетах) и понятную декларацию, где каждый доход отображается отдельной строкой, и которая предоставляет максимум
информации о всех полученных доходах (насколько это позволяет формат самой декларации). Никаких подтверждающих
документов при этом не прикладываем, чтобы лишний раз не вводить инспектора в заблуждение из-за особенностей отчетов
каждого брокера. Если же налоговая по какой-то причине решит, что подтверждающие документы ей все-таки нужны — тогда уже
отправляем вместе с длинной пояснительной запиской, в которой подробно расписываем, как надо интерпретировать информацию
в отчетах.

## Уведомление об открытии счета и отчет о движении денежных средств

Вся информация, включая сроки и формы для заполнения, [есть на сайте
ФНС](https://www.nalog.ru/rn77/related_activities/exchange_controls/).

Берем форму, заполняем там все поля кроме `SWIFT код или БИК`, `Дата выдачи разрешения`, `Номер разрешения`:
* Код вида документа: [21 (паспорт)](http://www.consultant.ru/document/cons_doc_LAW_283982/9ef6cf8ad5a3e982260724752a5307712d6b3d92/)
* Признак уведомления для резидента - физического лица: 1 – во исполнение [части 2 статьи 12 Федерального закона от 10.12.2003 No 173-ФЗ](http://www.consultant.ru/document/cons_doc_LAW_45458/0fb98bca6d0725e55c7306a484a3c51fd5636a62/)
* Код страны: [840 (США)](https://ru.wikipedia.org/wiki/Общероссийский_классификатор_стран_мира)
* Код валюты счета: [840 (USD)](https://index.minfin.com.ua/reference/currency/code/)

Данные об организации:

Interactive Brokers ([SEC:Interactive Brokers LLC](https://sec.report/CIK/0000922792)):
* Наименование: Interactive Brokers LLC
* Номер налогоплательщика: 133863700
* Адрес: Two Pickwick Plaza, Greenwich, CT 06830

Firstrade ([SEC: Firstrade Securities Inc.](https://sec.report/CIK/0000775397)):
* Наименование: Firstrade Securities Inc.
* Номер налогоплательщика: 112750321
* Адрес:30-50 Whitestone Expressway, Suite A301, Flushing, NY 11354
  
Лично у меня (и не только) через личный кабинет уведомление о движении денежных средств [не
приняли](https://www.banki.ru/forum/?PAGE_NAME=message&FID=21&TID=377931&MID=8514278#message8514278), хотя уведомление
об открытии счета принимали без проблем. В распечатанном виде лично в инспекции приняли без каких-либо вопросов.

### Откуда брать информацию о движении денежных средств

<a name="cash-flow"></a>
В программе есть команда `cash-flow`, которая изначально разрабатывалась с этой целью: выдать числа `остаток на начало
периода`, `зачислено`, `списано`, `остаток на конец периода` + детализацию по всем операциям.

Со своей задачей она справляется, но с одной оговоркой: т. к. при [возврате налогов](#dividend-reclassifications) в IB
указываются не реальные даты зачисления средств на счет, а даты выплаты дивиденда / первичного удержания налога, то
расчеты, выдаваемые `cash-flow` для периода 01.01.XXXX - 31.12.XXXX, расходятся на величину этих возвратов. Потом они,
конечно, сходятся (если брать другие периоды), но вот уже конкретно для отчета о движении средств их по этой причине
использовать довольно тяжело.

Поэтому в случае с IB я беру эту информацию из отчета, сверяя с тем, что выдает `cash-flow`:
* `Остаток денежных средств на счете на начало отчетного периода` — `Начальная сумма средств` в отчете
* `Зачислено денежных средств за отчетный период` — сумма всех положительных значений между `Начальная сумма средств` и `Остаток средств на конец периода`
* `Списано денежных средств за отчетный период` — сумма всех отрицательных значений между `Начальная сумма средств` и `Остаток средств на конец периода`
* `Остаток денежных средств на счете на конец отчетного периода` — `Остаток средств на конец периода` в отчете

В случае с Firstrade беру информацию из `cash-flow`, сверяя ее с отчетами на конкретные даты.