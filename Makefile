all:
	true

install:
	mkdir -p $(DESTDIR)/usr/bin/
	cargo fetch
	cargo build --release
	cp -vf target/release/pika-installer-gtk4 $(DESTDIR)/usr/bin/
	chmod 755 $(DESTDIR)/usr/bin/pika-installer-gtk4
	mkdir -p $(DESTDIR)/usr/lib/pika/pika-installer-gtk4/scripts/
	mkdir -p $(DESTDIR)/usr/share/glib-2.0/schemas/
	cp -rvf data/scripts/*.sh $(DESTDIR)/usr/lib/pika/pika-installer-gtk4/scripts/
	cp data/*.xml $(DESTDIR)/usr/share/glib-2.0/schemas/
	chmod 755 $(DESTDIR)/usr/lib/pika/pika-installer-gtk4/scripts/*.sh
	mkdir -p $(DESTDIR)/usr/share/applications
	mkdir -p $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
	#cp -vf data/pika-drivers.svg $(DESTDIR)/usr/share/icons/hicolor/scalable/apps/
	#cp -vf data/com.pika.drivers.desktop  $(DESTDIR)/usr/share/applications/
