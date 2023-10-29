import axios from 'axios';
import * as _ from 'lodash';
import moment from 'moment';
import * as bigJson from 'big-json';

async function fetchData() {
  const response = await axios.get('https://jsonplaceholder.typicode.com/posts');
  return response.data;
}

function processData(data: any) {
  const sortedData = _.sortBy(data, ['userId']);
  const groupedData = _.groupBy(sortedData, 'userId');
  
  _.forEach(groupedData, (value, key) => {
    const dates = _.map(value, 'date');
    const momentDates = _.map(dates, date => moment(date));
    console.log(`User ${key} has posts dating: ${momentDates}`);
  });
}

function generateBigObject() {
  const bigObject: any = {};
  for (let i = 0; i < 1e6; i++) {
    bigObject[i] = `some really long string that doesn't mean anything ${i}`;
  }
  return bigObject;
}

async function main() {
  const data = await fetchData();
  processData(data);
  
  const bigObject = generateBigObject();
  
  const stringifyStream = bigJson.createStringifyStream({
    body: bigObject
  });

  stringifyStream.on('data', (chunk) => {
    // Do something with the stringified chunk
  });
  
  // Add more logic here to further manipulate the data and consume more RAM
}

main().catch(console.error);
