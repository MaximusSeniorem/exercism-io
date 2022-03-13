#pragma once
namespace space_age {
    class space_age{
        private:
        unsigned long long int m_sec;

        static const float sm_earthYearSec;
        static const float sm_mercuryYearSec;
        static const float sm_venusYearSec;
        static const float sm_marsYearSec;
        static const float sm_jupiterYearSec;
        static const float sm_saturnYearSec;
        static const float sm_urectumYearSec;
        static const float sm_neptuneYearSec;

        public:
        space_age(unsigned long long int sec);

        unsigned long int seconds() const;
        float on_mercury() const;
        float on_venus() const;
        float on_earth() const;
        float on_mars() const;
        float on_jupiter() const;
        float on_saturn() const;
        float on_uranus() const;
        float on_neptune() const;
    };
}  // namespace space_age



