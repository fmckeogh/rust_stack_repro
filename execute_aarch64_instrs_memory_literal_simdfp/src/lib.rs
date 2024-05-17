#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use CheckFPEnabled64::*;
use V_set::*;
use CreateAccDescASIMD::*;
use Mem_read::*;
use SPESampleSIMDFPLoadStore::*;
use common::*;
pub fn execute_aarch64_instrs_memory_literal_simdfp<T: Tracer>(
    state: &mut State,
    tracer: &T,
    nontemporal: bool,
    offset: u64,
    size: i64,
    t: i64,
    tagchecked: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        data: Bits,
        address: u64,
        nontemporal: bool,
        offset: u64,
        size: i64,
        t: i64,
        tagchecked: bool,
    }
    let fn_state = FunctionState {
        nontemporal,
        offset,
        size,
        t,
        tagchecked,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #12744u : u32
        let s_0_0: u32 = 12744;
        // D s_0_1: read-reg s_0_0:u64
        let s_0_1: u64 = {
            let value = state.read_register::<u64>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 64u16);
        // D s_0_3: read-var offset:u64
        let s_0_3: u64 = fn_state.offset;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 64u16);
        // D s_0_5: add s_0_2 s_0_4
        let s_0_5: Bits = (s_0_2 + s_0_4);
        // D s_0_6: cast reint s_0_5 -> u64
        let s_0_6: u64 = (s_0_5.value() as u64);
        // D s_0_7: write-var address <= s_0_6
        fn_state.address = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckFPEnabled64(s_0_8)
        let s_0_9: () = CheckFPEnabled64(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: read-var nontemporal:u8
        let s_1_1: bool = fn_state.nontemporal;
        // D s_1_2: read-var tagchecked:u8
        let s_1_2: bool = fn_state.tagchecked;
        // D s_1_3: call CreateAccDescASIMD(s_1_0, s_1_1, s_1_2)
        let s_1_3: ProductType9878976b5bcce9c9 = CreateAccDescASIMD(
            state,
            tracer,
            s_1_0,
            s_1_1,
            s_1_2,
        );
        // D s_1_4: read-var size:i64
        let s_1_4: i64 = fn_state.size;
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: read-var address:u64
        let s_1_6: u64 = fn_state.address;
        // D s_1_7: call Mem_read(s_1_6, s_1_5, s_1_3)
        let s_1_7: Bits = Mem_read(state, tracer, s_1_6, s_1_5, s_1_3);
        // D s_1_8: write-var data <= s_1_7
        fn_state.data = s_1_7;
        // N s_1_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #8s : i
        let s_2_0: i128 = 8;
        // D s_2_1: read-var size:i64
        let s_2_1: i64 = fn_state.size;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: mul s_2_2 s_2_0
        let s_2_3: i128 = ((s_2_2) * (s_2_0));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: cast zx s_2_4 -> i
        let s_2_5: i128 = (i128::try_from(s_2_4).unwrap());
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: read-var t:i64
        let s_2_7: i64 = fn_state.t;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: read-var data:bv
        let s_2_9: Bits = fn_state.data;
        // D s_2_10: call V_set(s_2_8, s_2_6, s_2_9)
        let s_2_10: () = V_set(state, tracer, s_2_8, s_2_6, s_2_9);
        // C s_2_11: const #22416u : u32
        let s_2_11: u32 = 22416;
        // D s_2_12: read-reg s_2_11:u8
        let s_2_12: bool = {
            let value = state.read_register::<bool>(s_2_11 as isize);
            tracer.read_register(s_2_11 as isize, value);
            value
        };
        // N s_2_13: branch s_2_12 b4 b3
        if s_2_12 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call SPESampleSIMDFPLoadStore(s_4_0)
        let s_4_1: () = SPESampleSIMDFPLoadStore(state, tracer, s_4_0);
        // N s_4_2: return
        return;
    }
}
