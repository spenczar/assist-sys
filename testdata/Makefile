all: planets.dat asteroids.dat

planets.dat:
	curl -O https://ssd.jpl.nasa.gov/ftp/eph/planets/Linux/de440/linux_p1550p2650.440
	mv linux_p1550p2650.440 planets.dat

asteroids.dat:
	curl -O https://ssd.jpl.nasa.gov/ftp/eph/small_bodies/asteroids_de441/sb441-n16.bsp
	mv sb441-n16.bsp asteroids.dat

clean:
	rm -f planets.dat asteroids.dat

