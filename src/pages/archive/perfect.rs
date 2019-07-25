use seed::prelude::*;
use crate::Msg;

pub fn render() -> seed::dom_types::Node<Msg> {
    
    div![
        h1!["The Perfect Numbers"],
        br![],
        br![],
        br![],
        El::from_html("<table cellpadding=\"0\" cellspacing=\"0\" border=\"0\" class=\"perfecttable text\">
<tbody><tr>
	<td width=\"60\"><b>Rank</b></td>
	<td width=\"170\"><b>Perfect Number</b></td>
	<td width=\"80\"><b>Digits</b></td>
	<td width=\"290\"><b>Discoverer</b></td>
	<td align=\"center\" colspan=\"2\"><b>Downloads</b></td>
</tr>
<tr>
  <td>50</td>
  <td>2<sup>77232916</sup> × (2<sup>77232917</sup>-1)</td>
  <td>46498850</td>
  <td>2017 Jonathan Pace, George Woltman, Scott Kurowski, Aaron Blosser, et al..</td>
  <td></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/50.zip\">ZIP</a></td>
</tr>
<tr>
  <td>49</td>
  <td>2<sup>74207280</sup> × (2<sup>74207281</sup>-1)</td>
  <td>44677235</td>
  <td>2016 Cooper, Woltman, Kurowski, Blosser et al.</td>
  <td></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/49.zip\">ZIP</a></td>
</tr>
<tr>
  <td>48</td>
  <td>2<sup>57885160</sup> × (2<sup>57885161</sup>-1)</td>
  <td>34850340</td>
  <td>2013 Cooper, Woltman, Kurowski, et al.</td>
  <td></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/48.zip\">ZIP</a></td>
</tr>
<tr>
  <td>47</td>
  <td>2<sup>43112608</sup> × (2<sup>43112609</sup>-1)</td>
  <td>25956377</td>
  <td>2008 Smith, Woltman, Kurowski, et al.</td>
  <td></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/47.zip\">ZIP</a></td>
</tr>
<tr>
  <td>46</td>
  <td>2<sup>42643800</sup> × (2<sup>42643801</sup>-1)</td>
  <td>42643801</td>
  <td>2009 Strindmo, Woltman, Kurowski, et al.</td>
  <td></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/46.zip\">ZIP</a></td>
</tr>
<tr>
  <td>45</td>
  <td>2<sup>37156666</sup> × (2<sup>37156667</sup>-1)</td>
  <td>22370543</td>
  <td>2008 Elvenich, Woltman, Kurowski, et al.</td>
  <td></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/45.zip\">ZIP</a></td>
</tr>
<tr>
  <td>44</td>
  <td>2<sup>32582656</sup> × (2<sup>32582657</sup>-1)</td>
  <td>19616714</td>
  <td>2006 Cooper, Boone, Woltman, Kurowski, et al.</td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/44.txt\">TXT</a></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/44.zip\">ZIP</a></td>
</tr>
<tr>
  <td>43</td>
  <td>2<sup>30402456</sup> × (2<sup>30402457</sup>-1)</td>
  <td>18304103</td>
  <td>2005 Cooper, Boone, Woltman, Kurowski, et al.</td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/43.txt\">TXT</a></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/43.zip\">ZIP</a></td>
</tr>
<tr>
  <td>42</td>
  <td>2<sup>25964950</sup> × (2<sup>25964951</sup>-1)</td>
  <td>15632458</td>
  <td>2005 Nowak, Woltman, Kurowski, et. al.</td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/42.txt\">TXT</a></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/42.zip\">ZIP</a></td>
</tr>
<tr>
  <td>41</td>
  <td>2<sup>24036582</sup> × (2<sup>24036583</sup>-1)</td>
  <td>14471465</td>
  <td>2004 Findley, Woltman, Kurowski, et. al.</td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/41.txt\">TXT</a></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/41.zip\">ZIP</a></td>
</tr>
<tr>
  <td>40</td>
  <td>2<sup>20996010</sup> × (2<sup>20996011</sup>-1)</td>
  <td>12640858</td>
  <td>2003 Shafer, Woltman, Kurowski, et. al.</td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/40.txt\">TXT</a></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/40.zip\">ZIP</a></td>
</tr>
<tr>
  <td>39</td>
  <td>2<sup>13466916</sup> × (2<sup>13466917</sup>-1)</td>
  <td>8107892</td>
  <td>2001 Cameron, Woltman, Kurowski, et. al.</td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/39.txt\">TXT</a></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/39.zip\">ZIP</a></td>
</tr>
<tr>
  <td>38</td>
  <td>2<sup>6972592</sup> × (2<sup>6972593</sup>-1)</td>
  <td>4197919</td>
  <td>1999 Hajratwala, Woltman, Kurowski, et. al.</td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/38.txt\">TXT</a></td>
  <td><a href=\"//static.bigprimes.net/archive/perfect/38.zip\">ZIP</a></td>
</tr>
<tr>
	<td>37</td>
	<td>2<sup>3021376</sup> × (2<sup>3021377</sup>-1)</td>
	<td>1819050</td>
	<td>1998 Clarkson, Woltman, Kurowski, et. al.</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/37.txt\">TXT</a></td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/37.zip\">ZIP</a></td>
</tr>
<tr>
	<td>36</td>
	<td>2<sup>2976220</sup> × (2<sup>2976221</sup>-1)</td>
	<td>1791864</td>
	<td>1997 Spence, Woltman, et. al.</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/36.txt\">TXT</a></td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/36.zip\">ZIP</a></td>
</tr>
<tr>
	<td>35</td>
	<td>2<sup>1398268</sup> × (2<sup>1398269</sup>-1)</td>
	<td>841842</td>
	<td>1996 Armengaud, Woltman, et. al.</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/35.txt\">TXT</a></td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/35.zip\">ZIP</a></td>
</tr>
<tr>
	<td>34</td>
	<td>2<sup>1257786</sup> × (2<sup>1257787</sup>-1)</td>
	<td>757263</td>
	<td>1996 Slowinski&amp;Gage</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/34.txt\">TXT</a></td>
</tr>
<tr>
	<td>33</td>
	<td>2<sup>859432</sup> × (2<sup>859433</sup>-1)</td>
	<td>517430</td>
	<td>1994 Slowinski&amp;Gage</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/33.txt\">TXT</a></td>
</tr>
<tr>
	<td>32</td>
	<td>2<sup>756838</sup> × (2<sup>756839</sup>-1)</td>
	<td>455663</td>
	<td>1992 Slowinski&amp;Gage</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/32.txt\">TXT</a></td>
</tr>
<tr>
	<td>31</td>
	<td>2<sup>216090</sup> × (2<sup>216091</sup>-1)</td>
	<td>130100</td>
	<td>1985 Slowinski</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/31.txt\">TXT</a></td>
</tr>
<tr>
	<td>30</td>
	<td>2<sup>132048</sup> × (2<sup>132049</sup>-1)</td>
	<td>79502</td>
	<td>1983 Slowinski</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/30.txt\">TXT</a></td>
</tr>
<tr>
	<td>29</td>
	<td>2<sup>110502</sup> × (2<sup>110503</sup>-1)</td>
	<td>66530</td>
	<td>1988 Colquitt&amp;Welsh</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/29.txt\">TXT</a></td>
</tr>
<tr>
	<td>28</td>
	<td>2<sup>86242</sup> × (2<sup>86243</sup>-1)</td>
	<td>51924</td>
	<td>1982 Slowinski</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/28.txt\">TXT</a></td>
</tr>
<tr>
	<td>27</td>
	<td>2<sup>44496</sup> × (2<sup>44497</sup>-1)</td>
	<td>26790</td>
	<td>1979 Nelson&amp;Slowinski</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/27.txt\">TXT</a></td>
</tr>
<tr>
	<td>26</td>
	<td>2<sup>23208</sup> × (2<sup>23209</sup>-1)</td>
	<td>13973</td>
	<td>1979 Noll</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/26.txt\">TXT</a></td>
</tr>
<tr>
	<td>25</td>
	<td>2<sup>21700</sup> × (2<sup>21701</sup>-1)</td>
	<td>13066</td>
	<td>1978 Noll&amp;Nickel</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/25.txt\">TXT</a></td>
</tr>
<tr>
	<td>24</td>
	<td>2<sup>19936</sup> × (2<sup>19937</sup>-1)</td>
	<td>12003</td>
	<td>1971 Tuckerman</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/24.txt\">TXT</a></td>
</tr>
<tr>
	<td>23</td>
	<td>2<sup>11212</sup> × (2<sup>11213</sup>-1)</td>
	<td>6751</td>
	<td>1963 Gillies</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/23.txt\">TXT</a></td>
</tr>
<tr>
	<td>22</td>
	<td>2<sup>9940</sup> × (2<sup>9941</sup>-1)</td>
	<td>5985</td>
	<td>1963 Gillies</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/22.txt\">TXT</a></td>
</tr>
<tr>
	<td>21</td>
	<td>2<sup>9688</sup> × (2<sup>9689</sup>-1)</td>
	<td>5834</td>
	<td>1963 Gillies</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/21.txt\">TXT</a></td>
</tr>
<tr>
	<td>20</td>
	<td>2<sup>4422</sup> × (2<sup>4423</sup>-1)</td>
	<td>2663</td>
	<td>1961 Hurwitz</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/20.txt\">TXT</a></td>
</tr>
<tr>
	<td>19</td>
	<td>2<sup>4252</sup> × (2<sup>4253</sup>-1)</td>
	<td>2561</td>
	<td>1961 Hurwitz</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/19.txt\">TXT</a></td>
</tr>
<tr>
	<td>18</td>
	<td>2<sup>3216</sup> × (2<sup>3217</sup>-1)</td>
	<td>1937</td>
	<td>1957 Riesel</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/18.txt\">TXT</a></td>
</tr>
<tr>
	<td>17</td>
	<td>2<sup>2280</sup> × (2<sup>2281</sup>-1)</td>
	<td>1373</td>
	<td>1952 Robinson</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/17.txt\">TXT</a></td>
</tr>
<tr>
	<td>16</td>
	<td>2<sup>2202</sup> × (2<sup>2203</sup>-1)</td>
	<td>1327</td>
	<td>1952 Robinson</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/16.txt\">TXT</a></td>
</tr>
<tr>
	<td>15</td>
	<td>2<sup>1278</sup> × (2<sup>1279</sup>-1)</td>
	<td>770</td>
	<td>1952 Robinson</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/15.txt\">TXT</a></td>
</tr>
<tr>
	<td>14</td>
	<td>2<sup>606</sup> × (2<sup>607</sup>-1)</td>
	<td>366</td>
	<td>1952 Robinson</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/14.txt\">TXT</a></td>
</tr>
<tr>
	<td>13</td>
	<td>2<sup>520</sup> × (2<sup>521</sup>-1)</td>
	<td>314</td>
	<td>1952 Robinson</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/13.txt\">TXT</a></td>
</tr>
<tr>
	<td>12</td>
	<td>2<sup>126</sup> × (2<sup>127</sup>-1)</td>
	<td>77</td>
	<td>1876 Lucas</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/12.txt\">TXT</a></td>
</tr>
<tr>
	<td>11</td>
	<td>2<sup>106</sup> × (2<sup>107</sup>-1)</td>
	<td>65</td>
	<td>1914 Powers</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/11.txt\">TXT</a></td>
</tr>
<tr>
	<td>10</td>
	<td>2<sup>88</sup> × (2<sup>89</sup>-1)</td>
	<td>54</td>
	<td>1911 Powers</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/10.txt\">TXT</a></td>
</tr>
<tr>
	<td>9</td>
	<td>2<sup>60</sup> × (2<sup>61</sup>-1)</td>
	<td>37</td>
	<td>1883 Pervushin</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/9.txt\">TXT</a></td>
</tr>
<tr>
	<td>8</td>
	<td>2<sup>30</sup> × (2<sup>31</sup>-1)</td>
	<td>19</td>
	<td>1772 Euler</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/8.txt\">TXT</a></td>
</tr>
<tr>
	<td>7</td>
	<td>2<sup>18</sup> × (2<sup>19</sup>-1)</td>
	<td>12</td>
	<td>1588 Cataldi</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/7.txt\">TXT</a></td>
</tr>
<tr>
	<td>6</td>
	<td>2<sup>16</sup> × (2<sup>17</sup>-1)</td>
	<td>10</td>
	<td>1588 Cataldi</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/6.txt\">TXT</a></td>
</tr>
<tr>
	<td>5</td>
	<td>2<sup>12</sup> × (2<sup>13</sup>-1)</td>
	<td>8</td>
	<td>1456 ?</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/5.txt\">TXT</a></td>
</tr>
<tr>
	<td>4</td>
	<td>2<sup>6</sup> × (2<sup>7</sup>-1)</td>
	<td>4</td>
	<td>?</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/4.txt\">TXT</a></td>
</tr>
<tr>
	<td>3</td>
	<td>2<sup>4</sup> × (2<sup>5</sup>-1)</td>
	<td>3</td>
	<td>?</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/3.txt\">TXT</a></td>
</tr>
<tr>
	<td>2</td>
	<td>2<sup>2</sup> × (2<sup>3</sup>-1)</td>
	<td>2</td>
	<td>?</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/2.txt\">TXT</a></td>
</tr>
<tr>
	<td>1</td>
	<td>2<sup>1</sup> × (2<sup>2</sup>-1)</td>
	<td>1</td>
	<td>?</td>
	<td><a href=\"//static.bigprimes.net/archive/perfect/1.txt\">TXT</a></td>
</tr>
</tbody></table>"),
    ]
}