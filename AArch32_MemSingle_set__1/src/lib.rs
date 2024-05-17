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
use ClearExclusiveByAddress::*;
use IsFault__2::*;
use HandleExternalWriteAbort::*;
use AArch32_Abort::*;
use ProcessorID::*;
use IsAligned__1::*;
use IsFault::*;
use SPESampleLoadStore::*;
use AArch32_TranslateAddress::*;
use PhysMemWrite::*;
use common::*;
pub fn AArch32_MemSingle_set__1<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u32,
    size: i64,
    accdesc_in: ProductType9878976b5bcce9c9,
    aligned: bool,
    ispair: bool,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        accdesc: ProductType9878976b5bcce9c9,
        gs_31108: bool,
        ga_24228: ProductTypef170cab34335b70c,
        gs_31107: bool,
        gs_31105: bool,
        memaddrdesc: ProductTypece7c66ccb2cab13e,
        memstatus: ProductTypef8c3639b88223255,
        gs_31106: bool,
        address: u32,
        size: i64,
        accdesc_in: ProductType9878976b5bcce9c9,
        aligned: bool,
        ispair: bool,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        address,
        size,
        accdesc_in,
        aligned,
        ispair,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #1s : i
        let s_0_0: i128 = 1;
        // D s_0_1: read-var size:i64
        let s_0_1: i64 = fn_state.size;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b24 b1
        if s_0_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var size:i64
        let s_1_1: i64 = fn_state.size;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cmp-eq s_1_2 s_1_0
        let s_1_3: bool = ((s_1_2) == (s_1_0));
        // N s_1_4: branch s_1_3 b23 b2
        if s_1_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #4s : i
        let s_2_0: i128 = 4;
        // D s_2_1: read-var size:i64
        let s_2_1: i64 = fn_state.size;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: cmp-eq s_2_2 s_2_0
        let s_2_3: bool = ((s_2_2) == (s_2_0));
        // N s_2_4: branch s_2_3 b22 b3
        if s_2_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #8s : i
        let s_3_0: i128 = 8;
        // D s_3_1: read-var size:i64
        let s_3_1: i64 = fn_state.size;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: cmp-eq s_3_2 s_3_0
        let s_3_3: bool = ((s_3_2) == (s_3_0));
        // N s_3_4: branch s_3_3 b21 b4
        if s_3_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #16s : i
        let s_4_0: i128 = 16;
        // D s_4_1: read-var size:i64
        let s_4_1: i64 = fn_state.size;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cmp-eq s_4_2 s_4_0
        let s_4_3: bool = ((s_4_2) == (s_4_0));
        // D s_4_4: write-var gs#31105 <= s_4_3
        fn_state.gs_31105 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#31105:u8
        let s_5_0: bool = fn_state.gs_31105;
        // D s_5_1: write-var gs#31106 <= s_5_0
        fn_state.gs_31106 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#31106:u8
        let s_6_0: bool = fn_state.gs_31106;
        // D s_6_1: write-var gs#31107 <= s_6_0
        fn_state.gs_31107 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#31107:u8
        let s_7_0: bool = fn_state.gs_31107;
        // D s_7_1: write-var gs#31108 <= s_7_0
        fn_state.gs_31108 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#31108:u8
        let s_8_0: bool = fn_state.gs_31108;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // D s_8_2: read-var accdesc_in:struct
        let s_8_2: ProductType9878976b5bcce9c9 = fn_state.accdesc_in;
        // D s_8_3: write-var accdesc <= s_8_2
        fn_state.accdesc = s_8_2;
        // D s_8_4: read-var address:u32
        let s_8_4: u32 = fn_state.address;
        // D s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 32u16);
        // D s_8_6: read-var size:i64
        let s_8_6: i64 = fn_state.size;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: call IsAligned__1(s_8_5, s_8_7)
        let s_8_8: bool = IsAligned__1(state, tracer, s_8_5, s_8_7);
        // N s_8_9: assert s_8_8
        let s_8_9: () = assert!(s_8_8);
        // D s_8_10: read-var size:i64
        let s_8_10: i64 = fn_state.size;
        // D s_8_11: cast zx s_8_10 -> i
        let s_8_11: i128 = (i128::try_from(s_8_10).unwrap());
        // D s_8_12: read-var address:u32
        let s_8_12: u32 = fn_state.address;
        // D s_8_13: read-var accdesc:struct
        let s_8_13: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_8_14: read-var aligned:u8
        let s_8_14: bool = fn_state.aligned;
        // D s_8_15: call AArch32_TranslateAddress(s_8_12, s_8_13, s_8_14, s_8_11)
        let s_8_15: ProductTypece7c66ccb2cab13e = AArch32_TranslateAddress(
            state,
            tracer,
            s_8_12,
            s_8_13,
            s_8_14,
            s_8_11,
        );
        // D s_8_16: write-var memaddrdesc <= s_8_15
        fn_state.memaddrdesc = s_8_15;
        // N s_8_17: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var memaddrdesc:struct
        let s_9_0: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_9_1: call IsFault(s_9_0)
        let s_9_1: bool = IsFault(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b20 b10
        if s_9_1 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var memaddrdesc.2:struct
        let s_11_0: ProductTypef170cab34335b70c = fn_state.memaddrdesc._2;
        // D s_11_1: write-var ga#24228 <= s_11_0
        fn_state.ga_24228 = s_11_0;
        // D s_11_2: read-var ga#24228.5:struct
        let s_11_2: u32 = fn_state.ga_24228._5;
        // C s_11_3: const #0u : u32
        let s_11_3: u32 = 0;
        // D s_11_4: cmp-eq s_11_2 s_11_3
        let s_11_4: bool = ((s_11_2) == (s_11_3));
        // N s_11_5: branch s_11_4 b19 b12
        if s_11_4 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #22416u : u32
        let s_13_0: u32 = 22416;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: bool = {
            let value = state.read_register::<bool>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // N s_13_2: branch s_13_1 b18 b14
        if s_13_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // D s_15_1: create-sum enum = 0:"s_15_0"
        let s_15_1: SumTypeb20592b6489a79bd = SumTypeb20592b6489a79bd::_0(s_15_0);
        // D s_15_2: read-var size:i64
        let s_15_2: i64 = fn_state.size;
        // D s_15_3: cast zx s_15_2 -> i
        let s_15_3: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_4: read-var memaddrdesc:struct
        let s_15_4: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_15_5: read-var accdesc:struct
        let s_15_5: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_15_6: read-var value_name:bv
        let s_15_6: Bits = fn_state.value_name;
        // D s_15_7: call PhysMemWrite(s_15_4, s_15_3, s_15_5, s_15_1, s_15_6)
        let s_15_7: ProductTypef8c3639b88223255 = PhysMemWrite(
            state,
            tracer,
            s_15_4,
            s_15_3,
            s_15_5,
            s_15_1,
            s_15_6,
        );
        // D s_15_8: write-var memstatus <= s_15_7
        fn_state.memstatus = s_15_7;
        // D s_15_9: read-var memstatus:struct
        let s_15_9: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_15_10: call IsFault__2(s_15_9)
        let s_15_10: bool = IsFault__2(state, tracer, s_15_9);
        // N s_15_11: branch s_15_10 b17 b16
        if s_15_10 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var size:i64
        let s_17_0: i64 = fn_state.size;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: read-var memstatus:struct
        let s_17_2: ProductTypef8c3639b88223255 = fn_state.memstatus;
        // D s_17_3: read-var memaddrdesc:struct
        let s_17_3: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_17_4: read-var accdesc:struct
        let s_17_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_17_5: call HandleExternalWriteAbort(s_17_2, s_17_3, s_17_1, s_17_4)
        let s_17_5: () = HandleExternalWriteAbort(
            state,
            tracer,
            s_17_2,
            s_17_3,
            s_17_1,
            s_17_4,
        );
        // N s_17_6: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: read-var accdesc:struct
        let s_18_1: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_18_2: read-var memaddrdesc:struct
        let s_18_2: ProductTypece7c66ccb2cab13e = fn_state.memaddrdesc;
        // D s_18_3: call SPESampleLoadStore(s_18_0, s_18_1, s_18_2)
        let s_18_3: () = SPESampleLoadStore(state, tracer, s_18_0, s_18_1, s_18_2);
        // N s_18_4: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var memaddrdesc.3:struct
        let s_19_0: ProductTypeda0231e9dc169f81 = fn_state.memaddrdesc._3;
        // C s_19_1: const #() : ()
        let s_19_1: () = ();
        // S s_19_2: call ProcessorID(s_19_1)
        let s_19_2: i128 = ProcessorID(state, tracer, s_19_1);
        // D s_19_3: read-var size:i64
        let s_19_3: i64 = fn_state.size;
        // D s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (i128::try_from(s_19_3).unwrap());
        // D s_19_5: call ClearExclusiveByAddress(s_19_0, s_19_2, s_19_4)
        let s_19_5: () = ClearExclusiveByAddress(state, tracer, s_19_0, s_19_2, s_19_4);
        // N s_19_6: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var memaddrdesc.0:struct
        let s_20_0: ProductType1d757adad216cdef = fn_state.memaddrdesc._0;
        // D s_20_1: read-var address:u32
        let s_20_1: u32 = fn_state.address;
        // D s_20_2: call AArch32_Abort(s_20_1, s_20_0)
        let s_20_2: () = AArch32_Abort(state, tracer, s_20_1, s_20_0);
        // N s_20_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#31105 <= s_21_0
        fn_state.gs_31105 = s_21_0;
        // N s_21_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var gs#31106 <= s_22_0
        fn_state.gs_31106 = s_22_0;
        // N s_22_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #1u : u8
        let s_23_0: bool = true;
        // D s_23_1: write-var gs#31107 <= s_23_0
        fn_state.gs_31107 = s_23_0;
        // N s_23_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #1u : u8
        let s_24_0: bool = true;
        // D s_24_1: write-var gs#31108 <= s_24_0
        fn_state.gs_31108 = s_24_0;
        // N s_24_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
