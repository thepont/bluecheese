package dataclumps.case2;

import java.util.ArrayList;
import java.util.List;

class TwinCityWeatherSeriesBad {
  private List<Integer> weather1;
  private List<Integer> weather2;

  public TwinCityWeatherSeriesBad() {
    this.weather1 = new ArrayList<>();
    this.weather2 = new ArrayList<>();
  }

  public void addWeather1Temp(int number) {
    weather1.add(number);
  }

  public void addWeather2Temp(int number) {
    weather2.add(number);
  }

  public void printEachCityWeather() {
    System.out.println("Weather City 1: ");
    for (int number : weather1) {
      System.out.println(number);
    }
    System.out.println("Weather City 2: ");
    for (int number : weather2) {
      System.out.println(number);
    }
  }

  public List<Integer> getCity1WeatherList() {
      return weather1;
  }

  public List<Integer> getCity2WeatherList() {
      return weather2;
  }
}

class CompareWeatherPatternsBad {
  public boolean isWeather1Greater(List<Integer> weather1List, List<Integer> weather2List) {
        int sumWeather1 = 0;
        int sumWeather2 = 0;
        for (int weather1 : weather1List) {
            sumWeather1 += weather1;
        }
        for (int weather2 : weather2List) {
            sumWeather2 += weather2;
        }
        return sumWeather1 > sumWeather2;
  }

  public double averageAcross2Weathers(List<Integer> weather1List, List<Integer> weather2List) {
        int sumWeather = 0;
        for(int i = 0; i < weather1List.size(); i++) {
            sumWeather += weather1List.get(i);
            sumWeather += weather2List.get(i);
        }
        return (double) sumWeather / weather1List.size();
  }
}