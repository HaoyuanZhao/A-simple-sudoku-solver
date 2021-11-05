$(document).ready(function () {
  //Build the table:
  var content = "<table>";
  for (row = 0; row < 9; row++){
    content += "<tr>";
    for (column = 0; column < 9; column++){
      id = "cell-" + (row*9 + column).toString();
      content += '<td>' + '<input id=' + id + ' class="sudoku_elem" maxlength="1">' + '</td>'; 
    }
    content += "</tr>";
  }
  $('#table_start_point').append(content);

  // Only allow number inputs
  $('input[class^="sudoku_elem"]').change(function() {
    $(this).val($(this).val().match(/\d*\.?\d+/));
  });

});

function myParseInt(s,r,a) { return parseInt(s,10); }

function solve_sudoku() {
  var sudoku = document.getElementsByClassName("sudoku_elem");
  var input_index_array = [];

  var msg = [];
  for(i = 0; i < 81; i++){
    if (sudoku[i].value == "" || sudoku[i].value == "0"){
      msg.push(0);
    } else {
      msg.push(parseInt(sudoku[i].value));
      input_index_array.push(i);
    }
  } 

  axios.post('api/solve_sudoku', msg, {headers: { 'Content-Type': 'text/plain'}})
  .then(function (response){
      if (response.status == 202){
        alert("Ummm... seem like no solution for this Sudoku.");
        return;
      } else if (response.status == 203){
        alert("You are giving an illegal Sudoku inputs.");
        return;
      }
      // console.log("The raw_data is :", response.data);
      var int_list = (response.data + "").split(',').map(myParseInt);
      // console.log("The int_list is :", int_list);
      fill_the_table(int_list, input_index_array)
      document.getElementById("calculate_button").onclick = null;   //unable the button 
    })
    .catch(function (error){
      console.log("sudoku.js: ", error)
    });
}

function fill_the_table(data, inputs_array){
  for( i = 0; i < 81; i++){
    document.getElementById("cell-" + i.toString()).value = data[i].toString();
    document.getElementById("cell-" + i.toString()).disabled = true;
    if (inputs_array.find(x => x == i) !== undefined){
      document.getElementById("cell-" + i.toString()).style.backgroundColor = "grey";
    }
  }

}

function play_sudoku() {
  for( i = 0; i <81; i++){
    var temp = document.getElementById("cell-" + i.toString());
    temp.value = "";
    temp.style.backgroundColor = "white";
    temp.disabled = false;
  }
  var difficulty = document.getElementById("difficulty").value;
  axios.post('api/play_sudoku', difficulty, {headers: { 'Content-Type': 'text/plain'}})
  .then(function (response){
      var int_list = (response.data + "").split(',').map(myParseInt);
      for( i = 0; i <81; i++){
        if (int_list[i] !== 0){
          var temp = document.getElementById("cell-" + i.toString());
          temp.value = int_list[i].toString();
          temp.disabled = true;
          temp.style.backgroundColor = "grey";
        }
      }
    })
    .catch(function (error){
      console.log("sudoku.js: ", error)
    });
}

function check_answer(){
  var sudoku = document.getElementsByClassName("sudoku_elem");
  var input_index_array = [];

  var msg = [];
  for(i = 0; i < 81; i++){
    if (sudoku[i].value == "" || sudoku[i].value == "0"){
      msg.push(0);
    } else {
      msg.push(parseInt(sudoku[i].value));
      input_index_array.push(i);
    }
  } 

  axios.post('api/check_sudoku', msg, {headers: { 'Content-Type': 'text/plain'}})
  .then(function (response){
      if (response.data == "1"){
        alert("Congratulation! you get the correct answer!");
        document.getElementById("check_answer_button").onclick = null;
      } else{
        alert("Your answer is not correct...");
      }
    })
    .catch(function (error){
      console.log("sudoku.js: ", error)
    });
}

function reload_the_page(){
  window.location.reload();
}

function go_to_solve(){
  window.location.replace("/solve_sudoku");
}

function back_to_home(){
  window.location.replace("/");
}

function go_to_play(){
  window.location.replace("/play_sudoku");

}
