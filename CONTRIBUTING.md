# Örneklerle Rust Katkı Yönergeleri

Örneklerle Rust'ı(kısaca RBE ya da ÖR) daha iyi hale getirmekle ilgilendiğiniz için teşekkürler! Katkılarınızı almak bizi çok sevindirir. Katkıda bulunanların bu linkte veya bu depoda bulunan [`CODE_OF_CONDUCT.md`] dosyasındaki  bulunan [Rust Davranış Kuralları]na uymasını bekliyoruz. 

[Rust Davranış Kuralları]: https://www.rust-lang.org/policies/code-of-conduct
[`CODE_OF_CONDUCT.md`]: https://github.com/rustturkey/rust-by-example-tr/blob/master/CODE_OF_CONDUCT.md

## Lisans

RBE(ÖR) ve tüm katkılar MIT ve Apache 2.0 lisansları altında çift lisanslıdır. Daha fazla ayrıntı için lütfen [`LICENSE-MIT`] ve [`LICENSE-APACHE`] dosyalarına bakın.

[`LICENSE-MIT`]: https://github.com/rust-lang/rust-by-example/blob/master/LICENSE-MIT
[`LICENSE-APACHE`]: https://github.com/rust-lang/rust-by-example/blob/master/LICENSE-APACHE

## Çekme Talepleri(Pull Requests)

RBE(ÖR)'de değişiklik yapmak için lütfen GitHub'daki çekme taleplerini(pull request) `ana` şubeye(master branch) gönderin. 

Bunları inceleyeceğiz ve birleştireceğiz ya da değişiklik isteyeceğiz. Travis CI her şeyi test eder, böylece ondan da geri bildirim alabilirsiniz.

Bir çekme isteğine eklemeler veya başka değişiklikler yaparsanız, önceki işlemleri değiştirmekten veya sadece yenilerini eklemekten çekinmeyin. Birleştirmeden önce işlemleri üste taşımanızı(squash etmek) isteyebiliriz, bu değişebilir. 

## Sorun İzleyici

Sorun İzleyiciyi [GitHub'da](https://github.com/rust-lang/rust-by-example/issues) bulabilirsiniz. RBE(ÖR) ile ilgili bir sorun bulursanız lütfen buraya sorun olarak açın.

Takip eden etiketleri kullanıyoruz:

* `enhancement`: (artırma) Yeni bölümler veya fonksiyonlar için herhangi bir istek içindir.
* `bug`: (böcek) RBE(ÖR)'de olan yanlış veya çalışmayan her şey içindir.
* `discussion`: (tartışma) RBE(ÖR)'de bir şeyin gelişmesi hakkında bir tartışma; bu, yeni geliştirmelere veya hata sorunlarına yol açabilir.
* `E-mentor`: Bu sorun, kendisini yeni bir katılımcının düzeltmesine yardımcı olmaya adamıştır! Hem geliştirme hem de hata sorunlarını tatbik edebilir.

## Geliştirme İş Akışı

RBE(ÖR)'yi çalıştırmak için, [Rust'ı yükle], ve sonra:

```bash
$ git clone https://github.com/rust-lang/rust-by-example
$ cd rust-by-example
$ cargo install mdbook
$ mdbook build
```

[Rust'ı yükle]: http://rust-lang.org/install.html

Dosyalar en üst düzeydeki `book` dizininde olacaktır; `mdbook serve` içeriği web tarayıcınızda açacaktır.

Denemeleri çalıştırmak için:

```bash
$ mdbook test
```

Eğer yeni bir bölüm ekliyorsanız, eklemek için `src\SUMMARY.md` dosyasını düzenlemeniz gerekir. Mevcut olan bir örnekte ince ayar yapıyorsanız, ilgili dosyayı düzenlemeniz gerekir; bölümlerin dosyalarda nereye gittiğini görmek için 
`src\SUMMARY.md` dosyasını kontrol edin.
