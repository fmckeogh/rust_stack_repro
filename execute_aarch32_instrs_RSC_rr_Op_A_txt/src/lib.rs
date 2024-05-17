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
use R_read::*;
use R_set::*;
use Shift::*;
use AddWithCarry::*;
use common::*;
pub fn execute_aarch32_instrs_RSC_rr_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    s: i64,
    setflags: bool,
    shift_t: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_883106: ProductTyped54bc449dd09e5bd,
        nzcv: u8,
        d: i64,
        m: i64,
        n: i64,
        s: i64,
        setflags: bool,
        shift_t: u32,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        s,
        setflags,
        shift_t,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var s:i64
        let s_0_0: i64 = fn_state.s;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: call R_read(s_0_1)
        let s_0_2: u32 = R_read(state, tracer, s_0_1);
        // C s_0_3: const #0s : i
        let s_0_3: i128 = 0;
        // D s_0_4: cast zx s_0_2 -> bv
        let s_0_4: Bits = Bits::new(s_0_2 as u128, 32u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #7s : i
        let s_0_7: i128 = 7;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_3 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_3)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 8u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: read-var m:i64
        let s_0_14: i64 = fn_state.m;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: call R_read(s_0_15)
        let s_0_16: u32 = R_read(state, tracer, s_0_15);
        // C s_0_17: const #16971u : u32
        let s_0_17: u32 = 16971;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: bool = {
            let value = state.read_register::<bool>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_16 -> bv
        let s_0_19: Bits = Bits::new(s_0_16 as u128, 32u16);
        // D s_0_20: cast zx s_0_13 -> i
        let s_0_20: i128 = (i128::try_from(s_0_13).unwrap());
        // D s_0_21: read-var shift_t:u32
        let s_0_21: u32 = fn_state.shift_t;
        // D s_0_22: call Shift(s_0_19, s_0_21, s_0_20, s_0_18)
        let s_0_22: Bits = Shift(state, tracer, s_0_19, s_0_21, s_0_20, s_0_18);
        // D s_0_23: cast reint s_0_22 -> u32
        let s_0_23: u32 = (s_0_22.value() as u32);
        // D s_0_24: read-var n:i64
        let s_0_24: i64 = fn_state.n;
        // D s_0_25: cast zx s_0_24 -> i
        let s_0_25: i128 = (i128::try_from(s_0_24).unwrap());
        // D s_0_26: call R_read(s_0_25)
        let s_0_26: u32 = R_read(state, tracer, s_0_25);
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 32u16);
        // D s_0_28: not s_0_27
        let s_0_28: Bits = !s_0_27;
        // D s_0_29: cast reint s_0_28 -> u32
        let s_0_29: u32 = (s_0_28.value() as u32);
        // C s_0_30: const #16971u : u32
        let s_0_30: u32 = 16971;
        // D s_0_31: read-reg s_0_30:u8
        let s_0_31: bool = {
            let value = state.read_register::<bool>(s_0_30 as isize);
            tracer.read_register(s_0_30 as isize, value);
            value
        };
        // D s_0_32: cast zx s_0_29 -> bv
        let s_0_32: Bits = Bits::new(s_0_29 as u128, 32u16);
        // D s_0_33: cast zx s_0_23 -> bv
        let s_0_33: Bits = Bits::new(s_0_23 as u128, 32u16);
        // D s_0_34: call AddWithCarry(s_0_32, s_0_33, s_0_31)
        let s_0_34: ProductTyped54bc449dd09e5bd = AddWithCarry(
            state,
            tracer,
            s_0_32,
            s_0_33,
            s_0_31,
        );
        // D s_0_35: write-var gs#883106 <= s_0_34
        fn_state.gs_883106 = s_0_34;
        // D s_0_36: read-var gs#883106.0:struct
        let s_0_36: Bits = fn_state.gs_883106._0;
        // D s_0_37: cast reint s_0_36 -> u32
        let s_0_37: u32 = (s_0_36.value() as u32);
        // D s_0_38: read-var gs#883106.1:struct
        let s_0_38: u8 = fn_state.gs_883106._1;
        // D s_0_39: write-var nzcv <= s_0_38
        fn_state.nzcv = s_0_38;
        // D s_0_40: read-var d:i64
        let s_0_40: i64 = fn_state.d;
        // D s_0_41: cast zx s_0_40 -> i
        let s_0_41: i128 = (i128::try_from(s_0_40).unwrap());
        // D s_0_42: call R_set(s_0_41, s_0_37)
        let s_0_42: () = R_set(state, tracer, s_0_41, s_0_37);
        // D s_0_43: read-var setflags:u8
        let s_0_43: bool = fn_state.setflags;
        // N s_0_44: branch s_0_43 b2 b1
        if s_0_43 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #3s : i
        let s_2_0: i128 = 3;
        // D s_2_1: read-var nzcv:u8
        let s_2_1: u8 = fn_state.nzcv;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 4u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #0s : i
        let s_2_5: i128 = 0;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_0 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: bool = ((s_2_7.value()) != 0);
        // C s_2_9: const #16984u : u32
        let s_2_9: u32 = 16984;
        // N s_2_10: write-reg s_2_9 <= s_2_8
        let s_2_10: () = {
            state.write_register::<bool>(s_2_9 as isize, s_2_8);
            tracer.write_register(s_2_9 as isize, s_2_8);
        };
        // C s_2_11: const #2s : i
        let s_2_11: i128 = 2;
        // D s_2_12: read-var nzcv:u8
        let s_2_12: u8 = fn_state.nzcv;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 4u16);
        // C s_2_14: const #1s : i64
        let s_2_14: i64 = 1;
        // C s_2_15: cast zx s_2_14 -> i
        let s_2_15: i128 = (i128::try_from(s_2_14).unwrap());
        // C s_2_16: const #0s : i
        let s_2_16: i128 = 0;
        // C s_2_17: add s_2_16 s_2_15
        let s_2_17: i128 = (s_2_16 + s_2_15);
        // D s_2_18: bit-extract s_2_13 s_2_11 s_2_17
        let s_2_18: Bits = (Bits::new(
            ((s_2_13) >> (s_2_11)).value(),
            u16::try_from(s_2_17).unwrap(),
        ));
        // D s_2_19: cast reint s_2_18 -> u8
        let s_2_19: bool = ((s_2_18.value()) != 0);
        // C s_2_20: const #16997u : u32
        let s_2_20: u32 = 16997;
        // N s_2_21: write-reg s_2_20 <= s_2_19
        let s_2_21: () = {
            state.write_register::<bool>(s_2_20 as isize, s_2_19);
            tracer.write_register(s_2_20 as isize, s_2_19);
        };
        // C s_2_22: const #1s : i
        let s_2_22: i128 = 1;
        // D s_2_23: read-var nzcv:u8
        let s_2_23: u8 = fn_state.nzcv;
        // D s_2_24: cast zx s_2_23 -> bv
        let s_2_24: Bits = Bits::new(s_2_23 as u128, 4u16);
        // C s_2_25: const #1s : i64
        let s_2_25: i64 = 1;
        // C s_2_26: cast zx s_2_25 -> i
        let s_2_26: i128 = (i128::try_from(s_2_25).unwrap());
        // C s_2_27: const #0s : i
        let s_2_27: i128 = 0;
        // C s_2_28: add s_2_27 s_2_26
        let s_2_28: i128 = (s_2_27 + s_2_26);
        // D s_2_29: bit-extract s_2_24 s_2_22 s_2_28
        let s_2_29: Bits = (Bits::new(
            ((s_2_24) >> (s_2_22)).value(),
            u16::try_from(s_2_28).unwrap(),
        ));
        // D s_2_30: cast reint s_2_29 -> u8
        let s_2_30: bool = ((s_2_29.value()) != 0);
        // C s_2_31: const #16971u : u32
        let s_2_31: u32 = 16971;
        // N s_2_32: write-reg s_2_31 <= s_2_30
        let s_2_32: () = {
            state.write_register::<bool>(s_2_31 as isize, s_2_30);
            tracer.write_register(s_2_31 as isize, s_2_30);
        };
        // C s_2_33: const #0s : i
        let s_2_33: i128 = 0;
        // D s_2_34: read-var nzcv:u8
        let s_2_34: u8 = fn_state.nzcv;
        // D s_2_35: cast zx s_2_34 -> bv
        let s_2_35: Bits = Bits::new(s_2_34 as u128, 4u16);
        // C s_2_36: const #1s : i64
        let s_2_36: i64 = 1;
        // C s_2_37: cast zx s_2_36 -> i
        let s_2_37: i128 = (i128::try_from(s_2_36).unwrap());
        // C s_2_38: const #0s : i
        let s_2_38: i128 = 0;
        // C s_2_39: add s_2_38 s_2_37
        let s_2_39: i128 = (s_2_38 + s_2_37);
        // D s_2_40: bit-extract s_2_35 s_2_33 s_2_39
        let s_2_40: Bits = (Bits::new(
            ((s_2_35) >> (s_2_33)).value(),
            u16::try_from(s_2_39).unwrap(),
        ));
        // D s_2_41: cast reint s_2_40 -> u8
        let s_2_41: bool = ((s_2_40.value()) != 0);
        // C s_2_42: const #16996u : u32
        let s_2_42: u32 = 16996;
        // N s_2_43: write-reg s_2_42 <= s_2_41
        let s_2_43: () = {
            state.write_register::<bool>(s_2_42 as isize, s_2_41);
            tracer.write_register(s_2_42 as isize, s_2_41);
        };
        // N s_2_44: return
        return;
    }
}
