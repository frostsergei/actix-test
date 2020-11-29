use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "f64")]
enum Messages {
    RequestP,
    RequestV,
    RequestT
}

#[derive(Message)]
#[rtype(result = "f64")]
struct SetP(f64);

#[derive(Message)]
#[rtype(result = "f64")]
struct SetV(f64);

#[derive(Message)]
#[rtype(result = "f64")]
struct SetT(f64);

#[derive(Message)]
#[rtype(result = "Result<f64, ()>")]
struct ReadN;

#[derive(Clone, Copy)]
struct PActor{
    pressure: f64
}

#[derive(Clone, Copy)]
struct VActor{
    volume: f64
}

#[derive(Clone, Copy)]
struct TActor{
    temperature: f64
}

#[derive(Clone)]
struct NActor{
    nu: f64,
    pressure_actor: Box<Addr<PActor>>,
    volume_actor: Box<Addr<VActor>>, 
    temperature_actor: Box<Addr<TActor>>
}


impl Actor for NActor{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("NActor is alive");
        
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("NActor is stopped");
    }
}

impl Actor for PActor{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("PActor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("PActor is stopped");
    }
}

impl Actor for VActor{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("VActor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("VActor is stopped");
    }
}

impl Actor for TActor{
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("TActor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("TActor is stopped");
    }
}

impl Handler<Messages> for PActor{
    type Result = f64;
    
    fn handle(&mut self, _: Messages, _ctx: &mut Context<Self>) -> f64 {
        self.pressure
    }
}

impl Handler<SetP> for PActor{
    type Result = f64;

    fn handle(&mut self, msg: SetP, _ctx: &mut Context<Self>) -> Self::Result {
        self.pressure = msg.0;
        self.pressure
        
    }
}

impl Handler<Messages> for VActor{
    type Result = f64;

    fn handle(&mut self, _: Messages, _ctx: &mut Context<Self>) -> f64 {
        self.volume
    }
}

impl Handler<SetV> for VActor{
    type Result = f64;

    fn handle(&mut self, msg: SetV, _ctx: &mut Context<Self>) -> Self::Result {
        self.volume = msg.0;
        self.volume
        
    }
}

impl Handler<Messages> for TActor{
    type Result = f64;
    
    fn handle(&mut self, _: Messages, _ctx: &mut Context<Self>) -> f64 {
        1.0 / (self.temperature * 8.31)
    }
}

impl Handler<SetT> for TActor{
    type Result = f64;

    fn handle(&mut self, msg: SetT, _ctx: &mut Context<Self>) -> Self::Result {
        self.temperature = msg.0;
        self.temperature
        
    }
}

impl Handler<ReadN> for NActor{
    type Result = ResponseActFuture<Self, Result<f64, ()>>;
    
    fn handle(&mut self, _: ReadN, _: &mut Context<Self>) -> Self::Result {
        let p = *self.pressure_actor.clone();
        let v = *self.volume_actor.clone();
        let t = *self.temperature_actor.clone();

        Box::pin(
            async{
                calc(p, v, t).await                    
            }
            .into_actor(self)
            .map(|res, _act, _ctx|{
                _act.nu = res.clone();
                Ok(res)
            }),
        )           
    }
}

impl Handler<Messages> for NActor{
    type Result = f64;

    fn handle(&mut self, _: Messages, _ctx: &mut Context<Self>) -> Self::Result {
        self.nu
    }
}

async fn calc(p: Addr<PActor>, v: Addr<VActor>, t: Addr<TActor>) -> f64 {
    let pv = p.send(Messages::RequestP).await.unwrap();
    let vv = v.send(Messages::RequestV).await.unwrap();
    let tv = t.send(Messages::RequestT).await.unwrap();

    pv * vv *tv
}


#[actix_rt::main]
async fn main() {

    let pressure_address = Box::new(PActor{pressure:10.0}.start());
    let volume_address =  Box::new(VActor{volume:10.0}.start());
    let temperature_address =  Box::new(TActor{temperature:10.0}.start());
    
    let nact = Box::new(NActor {nu: 0.0, volume_actor: volume_address.clone(), pressure_actor: pressure_address.clone(), temperature_actor: temperature_address.clone()}.start()); 
    let n = nact.send(ReadN).await.unwrap().ok().unwrap();
    println!("{}",n);

    let _ = (*pressure_address).send(SetP(20.0)).await.unwrap();
    let n = nact.send(ReadN).await.unwrap().ok().unwrap();
    println!("{}",n);

    let _ = (*volume_address).send(SetV(20.0)).await.unwrap();
    let n = nact.send(ReadN).await.unwrap().ok().unwrap();
    println!("{}",n);

    let _ = (*temperature_address).send(SetT(5.0)).await.unwrap();
    let n = nact.send(ReadN).await.unwrap().ok().unwrap();
    println!("{}",n);
}