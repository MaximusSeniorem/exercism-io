#include "space_age.h"

namespace space_age {
        const float space_age::sm_earthYearSec = 3.1557600e7;
        const float space_age::sm_mercuryYearSec = sm_earthYearSec * 0.2408467;
        const float space_age::sm_venusYearSec = sm_earthYearSec * 0.61519726;
        const float space_age::sm_marsYearSec = sm_earthYearSec * 1.8808158;
        const float space_age::sm_jupiterYearSec = sm_earthYearSec * 11.862615;
        const float space_age::sm_saturnYearSec = sm_earthYearSec * 29.447498;
        const float space_age::sm_urectumYearSec = sm_earthYearSec * 84.016846;
        const float space_age::sm_neptuneYearSec = sm_earthYearSec * 164.79132;
        
        space_age::space_age(unsigned long long int sec) : m_sec(sec){}

        unsigned long int space_age::seconds() const {
            return m_sec;
        }

        float space_age::on_mercury() const {
            return m_sec/sm_mercuryYearSec;
        }

        float space_age::on_venus() const {
            return m_sec/sm_venusYearSec; 
        }

        float space_age::on_earth() const {
            return m_sec/sm_earthYearSec;
        }

        float space_age::on_mars() const {
            return m_sec/sm_marsYearSec; 
        }

        float space_age::on_jupiter() const {
            return m_sec/sm_jupiterYearSec; 
        }

        float space_age::on_saturn() const {
            return m_sec/sm_saturnYearSec; 
        }

        float space_age::on_uranus() const {
            return m_sec/sm_urectumYearSec; 
        }

        float space_age::on_neptune() const {
            return m_sec/sm_neptuneYearSec; 
        }

}  // namespace space_age
